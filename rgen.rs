use std::io::{BufferedReader, File, fs};
use std::os;
use std::str;

fn main() {
	//Ensure that the user gave the correct command line argument. 
	if !(os::args().len() >= 2) {
		println!("Usage: ./rgen <path to content>");
		return;
	}
	let path = Path::new(os::args().get(1).to_owned());
	//Make sure the user gave us a directory and not a file. 
	if !path.is_dir() {
		println!("Error: Not a directory. Usage: ./rgen <path to content>");
		return;
	}
	//Create the path to each of the types of data.
	let pathToContent = Path::new(path.as_str().unwrap() + "/content/");
	let pathToInclude = Path::new(path.as_str().unwrap() + "/include/");
	let pathToResources = Path::new(path.as_str().unwrap() + "/resources/");
	let pathToTemplates = Path::new(path.as_str().unwrap() + "/templates/");
	//Now create vectors containing paths to each of the individual files of each type. 
	let rawContentFiles: Vec<Path> = fs::walk_dir(&pathToContent).ok().unwrap().collect();
	let rawIncludeFiles: Vec<Path> = fs::walk_dir(&pathToInclude).ok().unwrap().collect();
	let rawResourceFiles: Vec<Path> = fs::walk_dir(&pathToResources).ok().unwrap().collect();
	let rawTemplateFiles: Vec<Path> = fs::walk_dir(&pathToTemplates).ok().unwrap().collect();

	//Remove directories and hidden files from the listing
	let mut contentFiles: Vec<Path> = Vec::new();
	let mut includeFiles: Vec<Path> = Vec::new();
	let mut resourceFiles: Vec<Path> = Vec::new();
	let mut templateFiles: Vec<Path> = Vec::new();
	//Print all files for testing purposes.
	//Note that this prints directories and hidden files. We'll have to check for these later.
	println!("Content Files:");
	for p in rawContentFiles.iter() {
		if !(p.is_dir() || p.filename_str().unwrap()[0] == 0x2E) {
			contentFiles.push(Path::new(p));
			println!("\t{}", str::from_utf8(p.as_vec()).unwrap());
		}
	}
	println!("Include Files:");
	for p in rawIncludeFiles.iter() {
		if !(p.is_dir() || p.filename_str().unwrap()[0] == 0x2E || p.filename_str().unwrap() == "vars.txt") {
			includeFiles.push(Path::new(p));
			println!("\t{}", str::from_utf8(p.as_vec()).unwrap());
		}
	}
	println!("Resource Files:");
	for p in rawResourceFiles.iter() {
		if !(p.is_dir() || p.filename_str().unwrap()[0] == 0x2E) {
			resourceFiles.push(Path::new(p));
			println!("\t{}", str::from_utf8(p.as_vec()).unwrap());
		}
	}
	println!("Template Files:");
	for p in rawTemplateFiles.iter() {
		if !(p.is_dir() || p.filename_str().unwrap()[0] == 0x2E) {
			templateFiles.push(Path::new(p));
			println!("\t{}", str::from_utf8(p.as_vec()).unwrap());
		}
	}

	//Map resource names
	let resourceNames = loadResourceNames(resourceFiles);

	//Load vars.txt into vars, a vector of string tuples. Matched with %var or {%var}.
	let vars: Vec<(~str,~str)> = loadVars(pathToInclude);

	//Load internal link names so that they can be replaced for includes. Matched with $link or {$link}
	let internalLinks: Vec<(~str,~str)> = loadLinks(contentFiles);

	//Process includes. Matched with {.include}
	let includes: Vec<(~str,~str)> = loadIncludes(includeFiles, &vars, &internalLinks);

	//Process global css/js
	let globalCSSJS: Vec<~str> = loadGlobalCSSJS(pathToTemplates);

	//Buffer template names first, then process
	let templates: Vec<Template> = loadTemplates(templateFiles, &vars, &internalLinks, &includes);

	//Generate content
}

fn loadResourceNames(resourceFiles: Vec<Path>) -> Vec<~str> {
	let mut resourceNames: Vec<~str> = Vec::new();
	for p in resourceFiles.iter() {
		resourceNames.push(p.filestem_str().unwrap().to_owned());
	}
	return resourceNames;
}

fn loadVars(pathToInclude: Path) -> Vec<(~str,~str)> {
	let mut vars: Vec<(~str,~str)> = Vec::new();
	let varsPath = Path::new(pathToInclude.as_str().unwrap() + "/vars.txt");
	let mut varReader = BufferedReader::new(File::open(&varsPath));
	for line in varReader.lines() {
		let st = line.unwrap().to_owned();
		let v: Vec<&str> = st.split_str(": ").collect();
		let temp: (~str, ~str) = (v.get(0).trim().to_owned(), v.get(1).trim().to_owned());
		vars.push(temp);
	}
	return vars;
}

fn loadIncludes(includeFiles: Vec<Path>, vars: &Vec<(~str,~str)>, internalLinks: &Vec<(~str,~str)>) -> Vec<(~str,~str)> {
	let mut returnVec: Vec<(~str,~str)> = Vec::new();
	for p in includeFiles.iter() {
		let fileName = p.filestem_str().unwrap().to_owned();
		let mut fileReader = BufferedReader::new(File::open(p));
		let fileContent = replaceVars(fileReader.read_to_str().unwrap().to_owned(), vars, internalLinks);
		//println!("{}", fileContent);
		returnVec.push((fileName, fileContent));
	}
	return returnVec;
}

//This method is slow. Fix it. 
fn loadLinks(contentFiles: Vec<Path>) -> Vec<(~str,~str)> {
	let mut returnVec: Vec<(~str,~str)> = Vec::new();
	for p in contentFiles.iter() {
		let mut fileReader = BufferedReader::new(File::open(p));
		let mut linkName = "".to_owned();
		let mut linkPath = "".to_owned();
		for line in fileReader.lines() {
			let st = line.unwrap();
			if st.starts_with("\tlinkName:") {
				linkName = st.split_str(":").last().unwrap().trim().to_owned();
			}
			else if st.starts_with("\tpath:") {
				linkPath = st.split_str(":").last().unwrap().trim().to_owned();
			}
			if linkName != "".to_owned() && linkPath != "".to_owned() {
				break;
			}
		}
		if linkName == "".to_owned() || linkPath == "".to_owned() {
			println!("Warning: linkName or linkPath for content file {} is not set.", p.filename_str().unwrap());
		}
		else {
			returnVec.push((linkName, linkPath));
		}
	}
	return returnVec;
}

fn loadGlobalCSSJS(pathToTemplates: Path) -> Vec<~str> {
	let mut returnVec: Vec<~str> = Vec::new();
	let globalPath = Path::new(pathToTemplates.as_str().unwrap() + "/globals.txt");
	let mut fileReader = BufferedReader::new(File::open(&globalPath));
	let line1 = fileReader.read_line().unwrap();
	let css = line1.trim() == "css";
	let mut js = line1.trim() == "js";
	while css {
		let nextLine = fileReader.read_line();
		match nextLine {
			Ok(tex) => {
				let texOwned = tex.trim();
				if texOwned == "js" {
					js = true;
					break;
				}
				else {
					returnVec.push("<link rel='stylesheet' type='text/css' href='resources/css/" + texOwned + "'>");
				}
			},
			Err(_) => { break }
		}
	}
	while js {
		let nextLine = fileReader.read_line();
		match nextLine {
			Ok(tex) => {
				let texOwned = tex.trim();
				returnVec.push("<script type='text/javascript' src='resources/js/" + texOwned + "'></script>");
			},
			Err(_) => { break }
		}
	}
	/*println!("Global includes:");
	for elem in returnVec.iter() {
		println!("{}", elem);
	}*/
	return returnVec;
}

struct Template {
	headData: Vec<~str>,
	blockTemplates: Vec<(~str,~str)>,
	content: ~str
}

enum TemplateStep {
	incCSS,
	incJS,
	blocks,
	content
}

fn loadTemplates(templateFiles: Vec<Path>, vars: &Vec<(~str,~str)>, internalLinks: &Vec<(~str,~str)>, includes: &Vec<(~str,~str)>) -> Vec<Template> {
	let mut returnVec: Vec<Template> = Vec::new();
	//Do stuff here

	return returnVec;
}

fn replaceVars(mut text: ~str, vars: &Vec<(~str,~str)>, internalLinks: &Vec<(~str,~str)>) -> ~str {
	for var in vars.iter() {
		let (ref a, ref b) = *var;
		text = text.replace("{%" + *a + "}", *b);
	}
	for link in internalLinks.iter() {
		let (ref a, ref b) = *link;
		text = text.replace("{$" + *a + "}", *b);
	}
	return text
}
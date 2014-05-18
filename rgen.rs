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
	let mut resourceNames: Vec<~str> = Vec::new();
	for p in resourceFiles.iter() {
		resourceNames.push(p.filestem_str().unwrap().to_owned());
	}

	//Load vars.txt into vars, a vector of string tuples.
	let mut vars: Vec<(~str,~str)> = Vec::new();
	let varsPath = Path::new(pathToInclude.as_str().unwrap() + "vars.txt");
	let mut varReader = BufferedReader::new(File::open(&varsPath));
	for line in varReader.lines() {
		let st = line.unwrap().to_owned();
		let v: Vec<&str> = st.split_str(": ").collect();
		let temp: (~str, ~str) = (v.get(0).to_owned(), v.get(1).to_owned());
		vars.push(temp);
	}

	//Process includes
	//Buffer template names first, then process
	//Generate content
}
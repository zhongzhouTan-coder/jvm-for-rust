use std::env;
use std::path::PathBuf;

use clap::Parser;

mod classloader;

#[derive(Parser, Debug, Default)]
#[clap(
    author = "Tang xuan",
    version,
    about = "A simple java virtual machine cli."
)]
struct Args {
    /// Specifies a list of directories, JAR files, and ZIP archives to search for class files.
    #[arg(long = "class-path")]
    classpath: std::path::PathBuf,

    /// Sets the fully qualified classname to execute.
    #[arg(long = "class-name")]
    classname: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    run_jvm(args.classpath, &args.classname)
}

fn run_jvm(classpath: std::path::PathBuf, classname: &str) {
    let java_home = env::var("JAVA_HOME").unwrap();
    let bootstrap_class_path = PathBuf::from(format!("{}/JRE/lib", java_home));
    let bootstrap_loader = classloader::loader::ClassLoader::new(bootstrap_class_path, None);
    let extension_class_path = PathBuf::from(format!("{}/JRE/lib/ext", java_home));
    let extension_class_loader = classloader::loader::ClassLoader::new(
        extension_class_path,
        Some(Box::new(bootstrap_loader)),
    );
    let application_class_path = classpath;
    let application_class_loader = classloader::loader::ClassLoader::new(
        application_class_path,
        Some(Box::new(extension_class_loader)),
    );
    println!("Running JVM with classpath: {:?}", application_class_loader);
    println!("Executing Java class: {}", classname);
}

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
    println!("Running JVM with classpath: {:?}", classpath);
    println!("Executing Java class: {}", classname);
}

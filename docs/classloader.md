# Java Virtual Machine Class Loader Implementation

## What does the Class Loader do ?

When we compile a source (.java) file, it gets converted into byte code as a .class file. Furthermore, when we try to use this class in our program, the class loader loads it into the main memory. Normally the class that contains the main() method is the first class to be loaded into the memory. The class loading process happens in three phases.

1. *Loading*
JVM uses the ClassLoader.loadClass() method for loading the class into memory.

2. *Linking*
Linking a class or interface involves combining the different elements and dependencies of the program together.

3. *Initialization*
Initialization involves executing the initialization code of the class or interface.

## Three class loader in java

1. *Bootstrap class loader*
Bootstrap class Loader is responsible for loading all core java API classes. Bootstrap Class Path is JDK/JRE/lib.

2. *Extension class loader*
Extension Class Loader is a child class of Bootstrap class loader which is  responsible for loading all classes from the extension class path in java. The Extension Class Path is : JDK/JRE/lib/ext.

3. *Application class loader*
Application Class Loader is a child class of Extension class loader which is responsible for loading classes from application class-path. The Application class path is our environment class path.

## What is the process of Java virtual Machine Startup

The Java Virtual Machine starts up by creating an initial class or interface using the bootstrap class loader or a user-defined class loader. The Java Virtual Machine then links the initial class or interface, initializes it, and invokes the public static method void main(String[]). The invocation of this method drives all further execution. Execution of the Java Virtual Machine instructions constituting the main method may cause linking (and consequently creation) of additional classes and interfaces, as well as invocation of additional methods.
The initial class or interface is specified in an implementation-dependent manner. For example, the initial class or interface could be provided as a command line argument. Alternatively, the implementation of the Java Virtual Machine could itself provide an initial class that sets up a class loader which in turn loads an application. Other choices of the initial class or interface are possible so long as they are consistent with the specification given in the previous paragraph.

## What we need to do in Class Creation and Loading process

Creation of a class or interface C denoted by the name N consists of the construction of an implementation-specific internal representation of C in the method area of the Java Virtual Machine
1. Class or interface creation is triggered by another class or interface D, whose run-time constant pool symbolically references C by means of the name N. 
2. If N does not denote an array class, then the Java Virtual Machine relies on a class loader to locate a binary representation for a class or interface called N.
3. Once a class loader has located a binary representation, it relies in turn on the Java Virtual Machine to derive the class or interface C from the binary representation, and then to create C in the method area.
Note: Array classes do not have an external binary representation; they are created by the Java Virtual Machine via a different process.

It should be clear that loading a class or interface is a joint effort between the Java Virtual Machine and a class loader (or multiple class loaders, if delegation occurs). The ultimate outcome of loading is that the Java Virtual Machine creates a class or interface in its method area, so it is often convenient to say that a class or interface is loaded and thereby created.

## What is the pricinpals which need to satisfied when design the class laoder

1. Given the same name, a good class loader should always return the same Class object.
2. If a class loader L1 delegates loading of a class C to another loader L2, then for any type T that occurs as the direct superclass or a direct superinterface of C, or as the type of a field in C, or as the type of a formal parameter of a method or constructor in C, or as a return type of a method in C, L1 and L2 should return the same Class object.
3. If a class loader L1 delegates loading of a class C to another loader L2, then for any type T that occurs as the direct superclass or a direct superinterface of C, or as the type of a field in C, or as the type of a formal parameter of a method or constructor in C, or as a return type of a method in C, L1 and L2 should return the same Class object.

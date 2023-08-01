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

## How does class loader work?

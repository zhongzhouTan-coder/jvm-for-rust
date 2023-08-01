# Project purpose

This is just a project for learning purpose, and the main goal of this
project is to learning java jvm and also rust language.

## Main Features Road Map

- [x] *Command Line*
  support classpath and classname
- [ ] *Class Loader* - Implement a simple class loader to load Java class files into memory.
- [ ] *Bytecode Interpreter* - Create a bytecode interpreter to execute Java bytecode. Interpretation is slower than Just-In-Time (JIT) compilation, but it simplifies the implementation.
- [ ] *Runtime Data Area* - Design basic data structures for the runtime data area, including the method area and the heap.
- [ ] *Bytecode Execution Engine* - Implement the core of the JVM, the bytecode execution engine. This engine interprets Java bytecode and executes the corresponding instructions.
- [ ] *Object Model* - Create data structures to represent Java objects, including classes, instances, and arrays.
- [ ] *Method Invocation* -  Implement the mechanism for method invocation, including handling method arguments and return values.
- [ ] *Exceptions and Error Handling* -  Design and implement the exception handling mechanism as specified by the JVM specification.
- [ ] *Garbage Collection* -  Develop a garbage collector to manage memory and automatically reclaim unused objects. Start with a basic algorithm like mark-and-sweep.
- [ ] *Multithreading* -  Implement support for basic multithreading with thread creation, synchronization, and context switching.
- [ ] *Java Native Interface (JNI)* -  If you plan to support native code integration, develop the JNI layer to enable Java code to call functions written in other languages like C or C++.
- [ ] *Java Standard Library* -  Include a minimal subset of the Java standard library to support basic Java programs.
- [ ] *Performance Optimization* - As you progress, focus on performance optimization. Consider techniques like Just-In-Time (JIT) compilation or other optimizations to improve execution speed.
- [ ] *Compatibility and Testing* - Test your JVM implementation against the Java Compatibility Test Suite (CTS) to ensure compatibility with existing Java codebases.
- [ ] *Documentation and Community* -  Document your progress, design decisions, and how to build and use your JVM. Engage with the Rust community and other JVM experts to gain insights and feedback.
- [ ] *Incremental Expansion* -  Building a full JVM is a substantial task, so consider starting with a minimal viable JVM and then gradually expanding it to support more advanced Java features and optimizations.

# Jvm design thinking

----------------------------------------------------------------------------------

## How java virtual machine work --- roughly analyze

### class file source format

* Class Files (.class)
* JAR Files(.jar)
* WAR Files(.war)
* EAR Files(,ear)
* Java Appletes
* Android APK Files(.apk)

we may support `Class Files` first and JAR `Files(.jar)` for class loader to parse class file

### how to load class file into memory   class-loader

function design for class loader

1. file_stream.rs

   provide many function to handle class file bytes

2. class

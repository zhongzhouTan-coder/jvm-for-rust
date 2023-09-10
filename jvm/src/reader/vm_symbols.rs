macro_rules! VM_SYMBOLS_STRUCT {
	($name:ident, { $(template($key:ident, $value:expr))* }) => {
		impl $name {
			$(
				pub fn $key() -> String {
					String::from($value)
				}
			)*
		}
	};
}

pub struct VmSymbols;

VM_SYMBOLS_STRUCT!(VmSymbols, {
    template(java_base, 								"java.base")
    template(java_lang_system, 							"java/lang/System")
    template(java_lang_object,                          "java/lang/Object")
    template(java_lang_class,                           "java/lang/Class")
    template(java_lang_package,                         "java/lang/Package")
    template(java_lang_module,                          "java/lang/Module")
    template(java_lang_string,                          "java/lang/String")
    template(java_lang_stringlatin1,                    "java/lang/StringLatin1")
    template(java_lang_stringutf16,                     "java/lang/StringUTF16")
    template(java_lang_thread,                          "java/lang/Thread")
    template(java_lang_threadgroup,                     "java/lang/ThreadGroup")
    template(java_lang_cloneable,                       "java/lang/Cloneable")
    template(java_lang_throwable,                       "java/lang/Throwable")
    template(java_lang_classloader,                     "java/lang/ClassLoader")
    template(java_lang_threaddeath,                     "java/lang/ThreadDeath")
    template(java_lang_boolean,                         "java/lang/Boolean")
    template(java_lang_character,                       "java/lang/Character")
    template(java_lang_character_charactercache,        "java/lang/Character$CharacterCache")
    template(java_lang_characterdatalatin1,             "java/lang/CharacterDataLatin1")
    template(java_lang_float,                           "java/lang/Float")
    template(java_lang_double,                          "java/lang/Double")
    template(java_lang_byte,                            "java/lang/Byte")
    template(java_lang_byte_bytecache,                  "java/lang/Byte$ByteCache")
    template(java_lang_short,                           "java/lang/Short")
    template(java_lang_short_shortcache,                "java/lang/Short$ShortCache")
    template(java_lang_integer,                         "java/lang/Integer")
    template(java_lang_integer_integercache,            "java/lang/Integer$IntegerCache")
    template(java_lang_long,                            "java/lang/Long")
    template(java_lang_long_longcache,                  "java/lang/Long$LongCache")
});

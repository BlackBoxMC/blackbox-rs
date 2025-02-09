#!/usr/bin/python
import json
import os
import pathlib
import shutil
import re
import multiprocessing
import copy

f = open("./spigot.json")
libraries = json.loads(f.read())


# files to write to disk
file_cache = {}

# rust's reserved words; we use this to rename any functions
reserved_words = ["as", "break", "const", "continue", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
                  "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
                  "abstract", "become", "box", "do", "final", "macro", "override", "priv", "tyof", "unsized", "virtual", "yield","async","args"]

library_resolves = {
    "net.md_5": "blackboxmc-rs-bungee",
    "org.bukkit": "blackboxmc-rs-bukkit",
    "java.util": "blackboxmc-rs-java/src/util",
    "java.lang": "blackboxmc-rs-java/src/lang",
}

parsed_classes = {}
excluded_classes = [
    "org.bukkit.plugin.SimplePluginManager",    # uses stuff that isn't being generated due to that java bug and i don't want to write an entire class binding for something that's getting deprecated anyways.
    "java.lang.JavaThread", "java.lang.JavaIterable", "java.lang.JavaRunnable", "java.lang.JavaCharSequence", "java.util.regex.JavaMatcher", "java.util.JavaObservable", "java.util.JavaFormatter", "java.lang.JavaException", "java.util.JavaResourceBundle", "java.lang.JavaThrowable", "java.lang.JavaCloneable", "java.lang.JavaComparable", "java.lang.JavaClass", "java.lang.JavaStringBuffer","java.lang.JavaStringBuilder",

    "java.lang.constant",
    "java.lang.annotation",
    "java.lang.Number",
    "java.lang.invoke",

    "java.io",
    "org.yaml",

    "java.util.concurrent",                     # i want to lessen my workload and binding this is going to be pointless since your only option of multithreading is through BukkitRunnables.
    "java.util.stream",                         # unneeded when java's iterator is bound.

    "net.md_5.bungee.chat.TranslationRegistry$TranslationProvider",
    "org.bukkit.plugin.SimpleServicesManager",

    "java.util.JavaLocale$Category", "java.util.JavaLocale$FilteringMode",

    # dear sweet baby jesus i just want to be done with this and i will remove half of the java.util
    # package if it means fucking getting this done.
    "java.util.PropertyResourceBundle", "java.util.Currency", "java.util.EnumMap", "java.util.Spliterators",
                        "java.util.Spliterators$AbstractIntSpliterator",
                        "java.util.Spliterators$AbstractLongSpliterator", "java.util.Spliterators$AbstractSpliterator",
                        "java.util.SplittableRandom", "java.util.AbstractMap$SimpleImmutableEntry", "java.util.Arrays", "java.util.GregorianCalendar",
    "java.util.NavigableMap", "java.util.NavigableSet",
    "java.util.Base64$Decoder", "java.util.Base64$Encoder", "java.util.BitSet",
                        "java.util.Calendar", "java.util.Calendar$Builder", "java.util.Collections",
                        "java.util.Dictionary",
                        "java.util.DoubleSummaryStatistics",
                        "java.util.EventListenerProxy", "java.util.EventObject", "java.util.FormattableFlags",
                        "java.util.Formatter", "java.util.GregorianCalendar", "java.util.IntSummaryStatistics",
                        "java.util.ListResourceBundle", "java.util.LongSummaryStatistics", "java.util.Objects",
                        "java.util.Observable",
                        "java.util.PriorityQueue",
                        "java.util.Properties", "java.util.PropertyPermission",
                        "java.util.ResourceBundle", "java.util.ResourceBundle$Control",
                        "java.util.Scanner", "java.util.ServiceLoader", "java.util.SimpleTimeZone",
                        "java.util.Stack", "java.util.StringJoiner",
                        "java.util.StringTokenizer", "java.util.Timer", "java.util.TimerTask", "java.util.TimeZone", "java.util.Spliterator", "java.util.Spliterator$OfDouble", "java.util.Spliterator$OfInt",
                        "java.util.Spliterator$OfLong", "java.util.Spliterator$OfPrimitive", "java.lang.StackTraceElement", "java.lang.Throwable",
                        "java.util.logging.LoggingMXBean", "java.util.logging.FileHandler",
                        "java.util.logging.LogManager",

    # tempoerary
    "java.util.JavaSortedMap", "java.util.JavaSortedSet"
]

interface_names = []
filled_once = []
bindings = {}
omitted_classes = []
enums = []

def library_name_no_extra_paths(libname):
    return re.sub("/src/(.*)","",libname)

def library_name_format(crate_name,library):
    l1 = library_resolves[crate_name]
    l2 = library_resolves[library]
    st1 = re.sub("/src/(.*)","::\\1",l1.replace("-","_").replace("_rs_","_"))
    st2 = re.sub("/src/(.*)","",l2.replace("-","_").replace("_rs_","_"))
    if st2.replace("::","") in st1:
        st1 = st1.replace(st2,"crate")
    return st1

def library_name_format_to_crate(libname):
    reg = re.search("/src/(.*)",libname)
    if reg is not None:
        return "crate::"+reg.group(1)
    else:
        return "crate"

def in_excluded_classes(cl):
    if cl in excluded_classes:
        return True
    for c in excluded_classes:
        if cl.startswith(c):
            return True
    return False

def mod_rs_folder_populate(dir):
    files = os.listdir(dir)
    f = open(dir+os.sep+"mod.rs", "a")
    f2 = open(dir+os.sep+"mod.rs", "r")
    for file in files:
        filename = str(file).lower()

        suffix = ""
        if filename.endswith(".rs"):
            suffix = ".rs"
        if filename.replace(".rs", "") in reserved_words:
            shutil.move(os.path.join(dir, filename+suffix),
                        os.path.join(dir, "mod_"+filename+suffix))
            filename = "mod_"+filename+suffix

        file3 = os.path.join(dir, filename)
        if os.path.isdir(file3):
            to_write = "pub mod "+filename+";\n"
            # check if the file already has this line because i don't even- what?
            what = list(filter(lambda f: "pub mod" in f, f2.readlines()))
            if to_write not in what:
                f.write(to_write)
                mod_rs_folder_populate(file3)
    f.close()
    f2.close()

def camel_case_to_snake_case(string):
    return re.sub(r"([a-z])([A-Z])", r"\1_\2",
                    string).lower()

def snake_case_to_camel_case(string):
    return re.sub(r"_([a-z])", lambda f: f.group(1).upper(),
                    string.lower()).replace("_","")

def func_signature_format(ty, increment, returning, options_start_at=-1):
    thing_start = ""
    thing = ""
    ty["type_name_resolved"] = ty["type_name_resolved"].replace("<T>","")
    internal_do_into = ty["type_name_resolved"].startswith("crate") or ty["type_name_resolved"].startswith("bukkit") or ty["type_name_resolved"].startswith("blackbox")
    is_string = ty["type_name_resolved"] == "String"
    internal = internal_do_into or ty["type_name_resolved"].startswith("jni")
    generic_letters = []
    generic_args = []
    if(ty["type_name_lhand"] == "" and not returning):
        thing_start += ty["type_name_resolved"]
    else:
        if not returning:
            thing_start += ty["type_name_lhand"]+": "

        if(options_start_at > -1):
            if increment >= options_start_at:
                thing += "std::option::Option<"

        if(ty["type_name_resolved"].startswith("&dyn")):
            old = ty["type_name_resolved"]
            letter = chr(ord('a')+increment).upper()
            ty["type_name_resolved"] = letter
            generic_letters.append(letter)
            generic_args.append(letter+": blackboxmc_general::JNIRaw<'mc> + "+old.replace("&dyn","")+"<'mc>")

        if ty["is_array"]:
            thing += "Vec<"
            thing += ty["type_name_resolved"]
            if(internal and "'mc" not in ty["type_name_resolved"]):
                thing += "<'mc>"
            thing += ">"
        else:
            if (internal_do_into or is_string) and not returning:
                thing += "impl Into<"
            
            if len(ty["generics"]) >= 1 and "Java" not in ty["type_name_resolved"]:
                thing += ty["type_name_alone"]+"<"
                j = []
                for g in ty["generics"]:
                    g = g["type_name_resolved"]
                    if len(g) >= 2:
                        if g.startswith("crate") or g.startswith("bukkit") or g.startswith("jni") or g.startswith("blackbox") or g == "String":
                            k = ""
                            if not returning and not g.startswith("jni"):
                                k += "impl Into<"
                            if "'mc" not in g and g != "String":
                                k += g+"<'mc>"
                            else:
                                k += g
                            if not returning and not g.startswith("jni"):
                                k += ">"
                            j.append(k)
                        else:
                            j.append(g)
                    else:
                        generic_letters.append(g)
                thing += ",".join(j)
                thing += ">"
            else:
                thing += ty["type_name_resolved"]
                if(internal and "'mc" not in thing):
                    thing += "<'mc>"
            if (internal_do_into or is_string) and not returning:
                thing += ">"
        if(options_start_at != -1):
            if increment >= options_start_at:
                thing += ">"
    thing = thing.replace("><",",")
    name_only = ""
    return {
        "result": thing_start+thing,
        "result_name_only": thing_start.split(":")[0],
        "result_type_only": thing,
        "generic_letters": generic_letters,
        "generic_args": generic_args,
    }

def argument_name_format(objects):
    parts = objects[1].split(".")
    name = parts[len(parts) - 1].replace("[]", "s").replace("$", "")
    return re.sub(r"([a-z])([A-Z])", r"\1_\2", name).lower()

def code_format(ty, prefix, n, var_prefix="val", arg="", is_array=False, options_start_at=-1, no_sig=False):
    if arg == "":
        if "type_name_lhand" in ty:
            arg = ty["type_name_lhand"]
        else:
            arg = "arg"+str(n)

    typ = ty["type_name_resolved"].replace("&dyn ","")

    res = []

    do_option = n >= options_start_at and options_start_at != -1

    let = java_letter_from_rust(ty["qualifiedName"],is_array)
    if do_option:
        res.append("if let Some(a) = "+arg+" {")
        if not no_sig:
            res.append("sig += \""+let+"\";")
        new_arg = "a"
    else:
        new_arg = arg
        if options_start_at != -1:
            res.append("sig += \""+let+"\";")

    
    class_name = ty["qualifiedName"].replace(".","/").replace("Java","")

    if is_array:
        res.append("let arr = ")
        new_new_arg = new_arg
        match(ty["type_name_alone"].replace("Java","")):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                res.append(prefix+".new_"+ty["qualifiedName"]+"_array("+new_arg+".len() as i32);")
                res.append("let mut vec = Vec::new();")
                new_new_arg = "*"+new_arg+".get(i).unwrap()"
            case _:
                res.append(prefix+".new_object_array("+new_arg+".len() as i32, \""+class_name+"\", jni::objects::JObject::null());")
                new_new_arg = new_arg+".get(i).unwrap()"
                if ty["type_name_alone"].endswith("String") or ty["type_name_alone"].endswith("Class"):
                    new_new_arg = new_new_arg+".clone()"
        res.append("let arr = "+prefix+".translate_error_no_gen(arr)?;")
        res.append("for i in 0.."+new_arg+".len() {")
        new_arg = new_new_arg
        

    if(typ.startswith("crate") or typ.startswith("blackbox")): # for internal types...
        if "JavaEnum" in typ:
            return None

        # get the original object.
        fcall = ""
        if not is_array:
            fcall += ".into()"
        fcall += ".jni_object().clone()"
        if ty["type_name_resolved"].startswith("&dyn"):
            fcall = ".jni_object().clone()"

        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(unsafe { jni::objects::JObject::from_raw("+new_arg+fcall+")});")
    else:
        match(ty["type_name_alone"].replace("Java","")):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                if is_array and "." not in ty["qualifiedName"]:
                    if "boolean" in ty["qualifiedName"]:
                        res.append("let "+var_prefix+"_"+str(n)+" = "+new_arg+" as u8;")
                    else:
                        res.append("let "+var_prefix+"_"+str(n)+" = "+new_arg+";")
                else:
                    v = java_type_from_rust(ty)
                    match(ty["qualifiedName"]):
                        case "boolean":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Bool("+new_arg+".into());")
                        case "byte":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Byte("+new_arg+");")
                        case "char":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Char("+new_arg+");")
                        case "double":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Double("+new_arg+");")
                        case "float":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Float("+new_arg+");")
                        case "int":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Int("+new_arg+");")
                        case "long":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Long("+new_arg+");")
                        case "short":
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Short("+new_arg+");")
                        case _:
                            res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+prefix+".new_object(\""+class_name+"\", \""+v["function_signature"]+"\", vec!["+new_arg+".into()])?);")
            case "String":
                a = new_arg
                if not is_array:
                    a += ".into()"
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(jni::objects::JObject::from("+prefix+".new_string("+a+")?));")
            case "jni::objects::JObject":
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+new_arg+");")
            case "jni::objects::JClass":
                a = new_arg
                if not is_array:
                    a += ".into()"
                res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object("+a+");")
            case "Vec":
                
                match ty["qualifiedName"]:
                    # TODO: convert this to use the new java.util bindings instad.
                    case "java.util.List":
                        c = [
                            "let raw_"+var_prefix+"_"+str(n)+" = "+prefix+".new_object(\"java/util/ArrayList\", \"()V\", vec![])?;",
                            "for v in "+new_arg+"{"
                        ]

                        if type(ty["generics"]) == dict:
                            generics = [ty["generics"]]
                        else:
                            generics = ty["generics"]

                        if len(generics) <= 0:
                            generics.append({
                                "generics": [],
                                "name": "Object",
                                "qualifiedName": "java.lang.Object",
                                "type": "java.lang.Object",
                                "type_name_resolved": "jni::objects::JObject"
                            })

                        t1 = java_type_from_rust(generics[0])["class_name"]

                        package_name = ".".join(filter(lambda f: f.islower(), generics[0]["qualifiedName"].split(".")))

                        co = code_format(ty={
                            "type_name_resolved": generics[0]["type_name_resolved"],
                            "type_name_lhand": "v",
                            "is_array": ty["is_array"],
                            "is_interface": ty["is_interface"],
                            "qualifiedName": t1,
                            "type_name_alone": generics[0]["type_name_resolved"],
                            "generics": generics,
                            "package_name": package_name,
                            "options_start_at": options_start_at,
                            "usage_unsafe": False,
                        }, prefix=prefix, n=0, var_prefix="map_val", is_array=is_array, options_start_at=options_start_at, no_sig=True)
                        if co is None:
                            print("co is none")
                            return None
                        c.append("\n\t\t".join(co))

                        c.append(
                            prefix+".call_method("+
                                "&raw_"+var_prefix+"_"+str(n)+","+
                                "\"add\","
                                "\"(Ljava/lang/Object;)Z\","+
                                "vec![jni::objects::JValueGen::from(map_val_0)]"
                            ")?;"
                        )

                        c.append("};")

                        c.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(raw_"+var_prefix+"_"+str(n)+");")

                        res += c

                    case _:
                        print("Unhandled map ty (arg):\t\t"+ty["qualifiedName"])
                        return None
            case _:
                print("Untranslated argument:\t\t"+ty["type_name_alone"],"\t\t",ty["qualifiedName"])
                return None

    if is_array:
        match(ty["type_name_alone"].replace("Java","")):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                res.append("vec.push("+var_prefix+"_"+str(n)+")")
                res.append("}")
                res.append(prefix+".set_"+ty["qualifiedName"]+"_array_region(&arr, 0, &vec)?;")
            case _:
                res.append(prefix+".set_object_array_element(&arr, i as i32, "+var_prefix+"_"+str(n)+".l()?)?;")
                res.append("}")
        res.append("let "+var_prefix+"_"+str(n)+" = jni::objects::JValueGen::Object(arr);")
    if options_start_at != -1:
        if not no_sig:
            if is_array:
                res.append("args.push("+var_prefix+"_"+str(n)+".l()?.into());")
            else:
                res.append("args.push("+var_prefix+"_"+str(n)+");")
    if do_option:
        res.append("}")

    return res

def return_format_one_liner(val, var_name):
    match val:
        case "()": return "()"
        case "u16": return var_name+".c()?"
        case "i8": return var_name+".b()?"
        case "i16": return var_name+".s()?"
        case "bool": return var_name+".z()?"
        case "i32": return var_name+".i()?"
        case "i64": return var_name+".j()?"
        case "f32": return var_name+".f()?"
        case "f64": return var_name+".d()?"

def return_format(return_group, prefix, static, method, obj_call, val, types, is_trait, options_start_at, is_constructor, nullable, library):
    if return_group["is_array"]:
        end_line = ".unwrap()"
    else:
        end_line = "?"
    code = []
    if options_start_at != -1:
        arr = "args"
    else:
        arr = "vec!["+",".join(types)+"]"
    if static:
        code.append("let cls = jni.find_class(\""+val["qualifiedName"].replace(".","/").replace("Java","")+"\"); let cls = jni.translate_error_with_class(cls)?;")
        if is_constructor:
            code.append("let res = "+prefix+".new_object(cls,")
        else:
            code.append("let res = "+prefix+".call_static_method(cls,\""+method["original_name"]+"\",")

        code.append("sig.as_str(),"+arr+");")

        if return_group["type_name_resolved"] != "()":
            code.append("let res = ")
        if is_constructor:
            code.append("jni.translate_error_no_gen(res)?;")
        else:
            code.append("jni.translate_error(res)?;")
    else:
        code.append(
            "let res = "+prefix+".call_method("+
                    "&"+obj_call+","+
                    "\""+method["original_name"]+"\",sig.as_str(),"+arr+");")
        if return_group["type_name_resolved"] != "()":
            code.append("let res = ")
        code.append(prefix+".translate_error(res)?;")

    # check if the returning object isn't a primitive.
    match return_group["type_name_resolved"]:
        case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
            nop = 0
        case _:
            if nullable:
                code.append("if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {return Ok(None);}")

    if static:
        val_1 = "jni"
        if is_constructor:
            val_2 = "res"
        else:
            val_2 = "obj"
    else:
        if is_trait:
            val_1 = "self.jni_ref()"
        else:
            val_1 = "self.0"
        if return_group["is_array"]:
            val_2 = "unsafe { jni::objects::JObject::from_raw(*res) }"
        else:
            val_2 = "unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) }"

    # primitive translation should not be done inside the java.lang bindings.
    skip_primitives = False
    if "lang" in library:
        if is_constructor:
            skip_primitives = True
        else:
            skip_primitives = False

    class_name = return_group["qualifiedName"].replace(".","/").replace("Java","")

    if return_group["is_array"]:
        code.append("let arr = ")
        match(return_group["type_name_alone"].replace("Java","")):
            case "bool" | "i8" | "char" | "f64" | "f32" | "i32" | "i64" | "i16" | "u16":
                proper = return_group["qualifiedName"][0].upper() + "".join(return_group["qualifiedName"][1:])
                code.append("Into::<jni::objects::J"+proper+"Array>::into(res.l()?);")
                code.append("""
                    if arr.is_null() {
                        return Ok(Vec::new());
                    }
                    unsafe {
                        Ok("""+prefix+""".get_array_elements(&jni::objects::JPrimitiveArray::from_raw(arr.clone()), jni::objects::ReleaseMode::CopyBack)?
                            .to_vec()
                        )
                        }
                        """)
                return "\n".join(code)
            case _:
                code.append("""
                            Into::<jni::objects::JObjectArray>::into(res.l()?);
                            let len = """+prefix+""".get_array_length(&arr)?;
                            let mut vec = Vec::new();
                            for i in 0..len {
                                let res = """+prefix+""".get_object_array_element(&arr, i)?;
                                vec.push({
                                    """)
        #res.append("let arr = "+prefix+".translate_error_no_gen(arr)?;")
        #res.append("for i in 0.."+new_arg+".len() {")
        #new_arg = new_new_arg
    if not skip_primitives:
        match return_group["type_name_resolved"]:
            case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
                if not return_group["is_array"]:
                    code.append("Ok(")
                if nullable:
                    code.append("Some(")
                code.append(return_format_one_liner(return_group["type_name_resolved"],"res"))
                if nullable:
                    code.append(")")
                if not return_group["is_array"]:
                    code.append(")")
                else:
                    code.append("""
                        });
                    }
                    Ok(vec)""")
                return "\n".join(code)
            case "String":
                if not return_group["is_array"]:
                    code.append("Ok(")
                if nullable:
                    code.append("Some(")
                the = "res.as_jni().l"
                if return_group["is_array"]:
                    the = "*res"
                code.append(prefix+
                            ".get_string(unsafe { &jni::objects::JString::from_raw("+the+") })"+end_line+
                            ".to_string_lossy()"+
                            ".to_string()"
                )
                if nullable:
                    code.append(")")
                if not return_group["is_array"]:
                    code.append(")")
                else:
                    code.append("""
                        });
                    }
                    Ok(vec)""")
                return "\n".join(code)
    match return_group["type_name_resolved"]:
        case "()":
            if not return_group["is_array"]:
                code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append(return_format_one_liner(return_group["type_name_resolved"],"res"))
            if nullable:
                code.append(")")
            if not return_group["is_array"]:
                code.append(")")
        case "jni::objects::JObject":
            if not return_group["is_array"]:
                code.append("Ok(")
            if nullable:
                code.append("Some(")
            if is_constructor:
                code.append("res")
            else:
                if return_group["is_array"]:
                    code.append("res")
                else:
                    code.append("res.l()?")
            if nullable:
                code.append(")")
            if not return_group["is_array"]:
                code.append(")")
        case "jni::objects::JClass":
            if not return_group["is_array"]:
                code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append("unsafe {"+
                        "jni::objects::JClass::from_raw(res.as_jni().l)"+
                        "}")
            if nullable:
                code.append(")")
            if not return_group["is_array"]:
                code.append(")")
        case "(u8, u8, u8)":
            code.append("let r = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getRed\", \"(V)I\", vec![]);"+
                        "            ;let r = "+prefix+".translate_error(r)?.i()? as u8;"+
                        "let g = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getGreen\", \"(V)I\", vec![])"+
                        "            ; let g = "+prefix+".translate_error(g)?.i()? as u8;"+
                        "let b = "+prefix+
                        "            .call_method(unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }, \"getBlue\", \"(V)I\", vec![])"+
                        "            ; let b = "+prefix+".translate_error(b)?.i()? as u8;")
            
            if not return_group["is_array"]:
                code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append("(r, g, b)")
            if nullable:
                code.append(")")
            code.append(")")
        case "Vec":
            code.append("let mut new_vec = Vec::new();")
            gen = return_group["generics"][0]["type_name_resolved"]
            match return_group["qualifiedName"]:
                case "java.util.Collection":
                    code.append("let col = blackboxmc_java::util::JavaCollection::from_raw(&"+
                                                prefix+",res.l()?)?;"+
                                "let iter = col.iterator()?;")
                case "java.util.List":
                    code.append("let list = blackboxmc_java::util::JavaList::from_raw(&"+val_1+", res.l()?)?;"+
                                "let iter = list.iterator()?;")
                case _:
                    print("Unhandled map ty (return):\t\t"+return_group["qualifiedName"])
                    return None
            code.append("while iter.has_next()? {"+
                "            let obj = iter.next()?;")
            match gen:
                case "()" | "u16" | "i8" | "i16" | "bool" | "i32" | "i64" | "f32" | "f64":
                    return None
                    #code.append("new_vec.push("+return_format_one_liner(gen,"obj")+")")
                case "jni::objects::JObject":
                    code.append("new_vec.push(obj);")
                case "jni::objects::JClass":
                    code.append("new_vec.push(unsafe {"+
                        "jni::objects::JClass::from_raw(*obj)"+
                        "})")
                case "String":
                    code.append("new_vec.push("+prefix+
                        ".get_string(unsafe { &jni::objects::JString::from_raw(*obj) })"+end_line+
                        ".to_string_lossy()"+
                        ".to_string());")
                case _:
                    gen = gen.replace("<","::<")
                    code.append("new_vec.push("+gen+"::from_raw(&"+val_1+",obj,)?);")
            code.append("};")
            if not return_group["is_array"]:
                code.append("Ok(")
            if nullable:
                code.append("Some(")
            code.append("new_vec")
            if nullable:
                code.append(")")
            if not return_group["is_array"]:
                code.append(")")
        case _:
            if return_group["type_name_resolved"] == "blackboxmc_java::lang::JavaEnum":
                return_group["type_name_resolved"] = "Self"
            if return_group["type_name_resolved"].startswith("&dyn "):
                what = return_group["type_name_alone"].split("::")
                name = what[len(what)-1]
                what.pop()
                return_group["type_name_resolved"] = "::".join(what)+name

            if static and not is_constructor and not return_group["is_array"]:
                code.append("let obj = res.l()?;")
            if return_group["is_array"]:
                val_2 = "res"

            if nullable:
                if not return_group["is_array"]:
                    code.append("Ok(")
                code.append("Some(")
            code.append(return_group["type_name_resolved"]+"::from_raw(&"+
                                                prefix+","+
                                                val_2)
            #if return_group["qualifiedName"] in enums:
            #    code.append(", "+return_group["type_name_resolved"]+"::from_string(variant_str).ok_or(eyre::eyre!(\"String gaven for variant was invalid\"))"+end_line)
            if nullable:
                if not return_group["is_array"]:
                    code.append(")?")
                code.append(")")
            code.append(")")
            if return_group["is_array"]:
                code.append("?")
    if return_group["is_array"]:
        code.append("""
            });
        }
        Ok(vec)""")

    return "\n".join(code)

def java_call_signature_format(types, return_group, is_constructor=False):
    results = []
    for ty in types:
        results.append(java_letter_from_rust(ty["qualifiedName"],ty["is_array"]))
    if is_constructor:
        return "("+"".join(results)+")V"
    else:
        return "("+"".join(results)+")"+java_letter_from_rust(return_group["qualifiedName"],return_group["is_array"])

def correct_question_mark(ty, method, i, returning, library, is_constructor):
    name = ty
    if "," in name:
        names = name.split(",")
    else:
        names = [name]
    final = []
    for name in names:
        if "?" in name:
            name = name.replace("? extends","").replace("? super","").replace("<?>","").replace(" ","")
            # we don't support further generics at this point.
            gen2 = re.search("<(.*?)>", name)
            if gen2 is not None:
                return None
            # if its still there at this point, move on.
            if "?" in name:
                return None
        grp = java_type_to_rust("", name, method, i, returning, library, is_constructor=False, skip_vec=True)
        if grp is None:
            return None
        final.append(grp)
    return final


def java_type_to_rust_primitive(ty):
    type_name_resolved = ""
    match ty.replace("Java",""):
        case "void":
            type_name_resolved = "()"
        case "char" | "java.lang.Character":
            type_name_resolved = "u16"
        case "java.lang.Byte" | "byte":
            type_name_resolved = "i8"
        case "java.lang.Short" | "short":
            type_name_resolved = "i16"
        case "java.lang.Boolean" | "boolean":
            type_name_resolved = "bool"
        case "java.lang.Integer" | "int":
            type_name_resolved = "i32"
        case "java.lang.Long" | "long":
            type_name_resolved = "i64"
        case "java.lang.Float" | "float":
            type_name_resolved = "f32"
        case "java.lang.Double" | "double":
            type_name_resolved = "f64"
        case "java.lang.String":
            type_name_resolved = "String"
    if type_name_resolved != "":
        return type_name_resolved
    else:
        return None

def java_type_to_rust(argname, ty, method, i, returning, library, is_constructor=False, skip_vec=False):
    if method is not None and not is_constructor:
        if returning:
            if "returnType" not in method["method"]:
                if "generics" in method["method"]:
                    generics_ = method["method"]["generics"]
                else:
                    generics_ = []
            else:
                if "generics" in method["method"]["returnType"]:
                    generics_ = method["method"]["returnType"]["generics"]
                else:
                    generics_ = []
        else:
            if i <= len(method["method"]["parameters"]):
                if "generics" in method["method"]["parameters"][i]:
                    generics_ = method["method"]["parameters"][i]["generics"]
                else:
                    generics_ = []
            else:
                generics_ = []
    else:
        generics_ = []

    for gen in generics_:
        g = correct_question_mark(gen["type"],None, 0, True, library, False)
        if g is None:
            return None
        g = g[0]["type_name_resolved"]
        gen["type_name_resolved"] = g

    generics = []
    type_name_resolved = "jni::objects::JObject"
    is_array = False
    is_interface = False
    if ty.endswith("[]"):
        ty = ty.replace("[]", "")
        is_array = True

    if not isinstance(library, bool):
        if library.startswith("java"):
            ty = "Java"+ty

    type_alone = ""
    

    usage_unsafe = False

    if ty.replace("com.destroystokyo.paper","org.bukkit") in interface_names:
        is_interface = True

    qualifiedName = ty.replace("Java","")
    skip_primitives = False
    if "lang" in library:
        if is_constructor:
            skip_primitives = True
        else:
            skip_primitives = False

    if not skip_primitives:
        t = java_type_to_rust_primitive(ty)
        if t is not None:
            type_name_resolved = t
    if type_name_resolved == "jni::objects::JObject":
        match ty:
            case "java.lang.Class":
                type_name_resolved = "jni::objects::JClass"
            case "java.awt.Color":
                type_name_resolved = "(u8, u8, u8)"
            case "java.lang.Object" | "java.lang.reflect.Type" | "java.awt.Image" | "java.awt.image.BufferedImage":
                type_name_resolved = "jni::objects::JObject"
            case "java.util.List" | "java.util.Collection":
                if skip_vec:
                    return None
                type_name_resolved = "Vec"
                generics = generics_
            #case "java.util.HashMap":
            #    if skip_vec:
            #        return None
            #    type_name_resolved = "std::collections::HashMap"
            #    parts = []
            #    grp = correct_question_mark(argname, parameter_ty, method, i, returning, library, is_constructor)
            #    if grp is None:
            #        return None
            #    parts += grp
            #    generics = parts
            case _:
                ty = ty.replace("Javajava","java")
                crate_name = ".".join(
                    filter(lambda f: f.lower() == f, ty.split(".")))

                while crate_name not in library_resolves and crate_name != "":
                    parts = crate_name.split(".")
                    parts.pop()
                    crate_name = ".".join(parts)

                if crate_name == "":
                    usage_unsafe = True
                else:
                    if (library_name_format(crate_name,library) == library_name_format("java.util",library)) or (library_name_format(crate_name,library) == library_name_format("java.lang",library)):
                        class_name = "".join(filter(lambda f: f[0].upper() == f[0], ty.split(".")))
                        what = ty
                        ty = ty.replace(class_name, "Java"+class_name)

                    to_replace =  library_name_format(crate_name,library)

                    type_name_resolved = ty.replace(
                        crate_name, to_replace).replace("-", "_").replace("$", "")
                    
                    lower_part = "::".join(filter(lambda f: f.lower() == f,type_name_resolved.split(".")))
                    upper_part = "".join(filter(lambda f: f.lower() != f,type_name_resolved.split(".")))

                    type_name_resolved = lower_part+"::"+upper_part
                    if "JavaObject" in type_name_resolved:
                        type_name_resolved = "jni::objects::JObject"
    if type_alone == "":
        type_alone = type_name_resolved

    parts = type_name_resolved.split("::")
    for p in parts:
        if p in reserved_words:
            type_name_resolved = type_name_resolved.replace(
                p, "mod_"+p)
    if argname in reserved_words:
        argname = "val_"+argname

    # camel case to snake case
    type_name_lhand = camel_case_to_snake_case(argname).replace("$", "")
    if type_name_lhand in reserved_words:
        type_name_lhand = "arg_"+type_name_lhand

    if method is not None:
        if "options_start_at" in method:
            options_start_at = method["options_start_at"]
        else:
            options_start_at = -1
    else:
        options_start_at = -1
    package_name = ".".join(filter(lambda f: f.islower(), qualifiedName.split(".")))

    if(in_excluded_classes(ty)):
        return None

    return {
        "type_name_resolved": type_name_resolved,
        "type_name_lhand": type_name_lhand,
        "is_array": is_array,
        "is_interface": is_interface,
        "package_name": package_name,
        "qualifiedName": qualifiedName,
        "type_name_alone": type_alone,
        "generics": generics,
        "options_start_at": options_start_at,
        "usage_unsafe": usage_unsafe,
    }

def java_type_from_rust(ty):
    package_name = ".".join(filter(lambda f: f.islower(), ty["qualifiedName"].split(".")))
    match(ty["qualifiedName"]):
        case "bool":
            class_name = "boolean"
            function_signature = "(Z)V"
        case "byte":
            class_name = "byte"
            function_signature = "(B)V"
        case "u16":
            class_name = "char"
            function_signature = "(C)V"
        case "i16":
            class_name = "short"
            function_signature = "(S)V"
        case "i32":
            class_name = "int"
            function_signature = "(I)V"
        case "i64":
            class_name = "long"
            function_signature = "(J)V"
        case "f32":
            class_name = "float"
            function_signature = "(F)V"
        case "f64":
            class_name = "double"
            function_signature = "(D)V"
        case _:
            class_name = (package_name.replace(".","/"))+"/"+ty["qualifiedName"]
            function_signature = "(Ljava/Lang/Object;)V"
    return {
        "class_name": class_name,
        "function_signature": function_signature
    }

def java_letter_from_rust(ty,is_array):
    res = ""
    match(ty.replace("Java","")):
        case "boolean":
            res = "Z"
        case "byte":
            res = "B"
        case "char":
            res = "C"
        case "short":
            res = "S"
        case "int":
            res = "I"
        case "long":
            res = "J"
        case "float":
            res = "F"
        case "double":
            res = "D"
        case "void":
            res = "V"
        case "":
            res = ""
        case _:
            res = "L"+ty.replace(".","/").replace("Java","")+";"
    if is_array:
        return "["+res
    else:
        return res

def gen_to_string_func(name, static):
    if static:
        call = "Self::internal_to_string(&self.0)"
    else:
        call = "self.to_string()"
    return """
        impl<'mc> std::string::ToString for """+name+"""<'mc> {
            fn to_string(&self) -> String {
                match &self.internal_to_string() {
                    Ok(a) => a.clone(),
                    Err(err) => format!(
                        "Error calling """+name+""".toString: {}",
                        err
                    ),
                }
            }
        }
        """

def gen_instance_of_func(mod_path):
    impl_signature = []
    impl_signature.append("""
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error>  {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
    """)
    for impl in impl_signature:
        file_cache[mod_path].append(impl)

def gen_jniraw_impl(name, is_enum, mod_path, full_name, variants):
    impl_signature = []
    impl_signature.append("""
    impl<'mc> JNIRaw<'mc> for """+name+"""<'mc> {
        fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        """)
    if is_enum:
        impl_signature.append("match self {")
        for (_,val_proper) in variants:
            impl_signature.append("Self::"+val_proper+" { inner } => inner.0.clone(),")
        impl_signature.append("}")
    else:
        impl_signature.append("self.0.clone()")
    impl_signature.append("}")
    impl_signature.append("fn jni_object(&self) -> jni::objects::JObject<'mc> {")
    if is_enum:
        impl_signature.append("match self {")
        for (_,val_proper) in variants:
            impl_signature.append("Self::"+val_proper+" { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },")
        impl_signature.append("}")
    else:
        impl_signature.append("unsafe { jni::objects::JObject::from_raw(self.1.clone()) }")
    impl_signature.append("}")
    impl_signature.append("}")

    impl_signature.append(gen_jni_instantiatable(name,full_name,is_enum,variants))

    for impl in impl_signature:
        file_cache[mod_path].append(impl)

    if is_enum:
        gen_jniraw_impl(name+"Struct",False,mod_path,full_name,variants)

def gen_jni_instantiatable(name,full_name,is_enum,variants):
    st = """impl<'mc> JNIInstantiatable<'mc> for """+name+"""<'mc> {
        fn from_raw(
            env: &blackboxmc_general::SharedJNIEnv<'mc>,
            obj: jni::objects::JObject<'mc>,
        ) -> Result<Self, Box<dyn std::error::Error>> {
            if obj.is_null() {
                return Err(eyre::eyre!(
                    \"Tried to instantiate """+name+""" from null object.\")
                .into());
            }
            let (valid, name) = env.validate_name(&obj, \""""+full_name.replace(".","/")+"""\")?;
            if !valid {
                Err(eyre::eyre!(
                    "Invalid argument passed. Expected a """+name+""" object, got {}",
                    name
                )
                .into())
            } else {
    """
    if is_enum:
        st += """
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    """
        for (v,val_proper) in variants:
            st += "\""+v+"\" => Ok("+name+"::"+val_proper+" { inner: "+name+"Struct::from_raw(env,obj)?}),"
        st += "_ => Err(eyre::eyre!(\"String gaven for variant was invalid\").into())"
        st += "}"
    else:
        st += "Ok(Self(env.clone(), obj))"
    st += """
            }
        }
    }
    """
    return st
def parse_methods(library,name,val,mod_path,is_trait,is_trait_decl,is_constructor,dummy,parsed_names):
    packageName = ".".join(list(filter(lambda f: f.lower() == f,val["qualifiedName"].split("."))))
    
    if is_constructor:
        methods = val["constructors"]
    else:
        methods = val["methods"]
        if not dummy:
            method_names = list(map(lambda f: f["name"],methods))

            interfaces = list(map(lambda f: f["qualifiedName"],val["interfaces"]))
            for inte in interfaces:
                if inte in libraries[library]:
                    interface = libraries[library][inte]
                    for int in interface["methods"]:
                        if int["name"] in method_names:
                            if "Override" in str(int["annotations"]):
                                methods = list(filter(lambda f: f["name"] != int["name"],methods))
                    methods += interface["methods"]
        
    og_name = name
    impl_signature = []
    extern_signature = []
    methods = filter(lambda f: f["name"] != "valueOf",methods)

    has_to_string = False
    has_to_string_is_static = False

    names = {}
    methods_ = {}
    for method in methods:
        if "modifiers" in method:
            if("public" not in method["modifiers"]): # skip private methods
                continue
            if("abstract" in method["modifiers"]): # skip abstract methods
                continue

        if is_constructor:
            mname = "new"
        else:
            mname = method["name"]

        og_mname = mname

        # camel case to snake case
        mname = camel_case_to_snake_case(mname).replace("$", "")

        if len(method["parameters"]) <= 0 and mname != "get_header":
            mname = mname.replace("get_","")
            

        if mname in reserved_words:
            mname = "get_"+mname

        names[og_mname] = mname
        if mname in methods_:
            methods_[mname].append(method)
        else:
            methods_[mname] = [method]

    new_methods = {}
    for k in names:
        name = names[k]
        method = methods_[name]

        if len(method) > 1:
            m = list(filter(lambda f: "Override" in str(f),methods_[name]))
            if len(m) >= 1:
                method = m

        # are there more then one method with this name?
        if len(method) > 1:
            # ok time for some surgery then!

            can_use_options = True
            last_method = None

            # sort the arrays by length
            method_first_arg = ""
            method_map = {}
            method_buildup = []
            options_start_at = 0
            methods__ = sorted(method, key = lambda key : len(key["parameters"]))
            n = 0
            for m in methods__:
                if "public" not in m["modifiers"]:
                    continue
                if "abstract" in m["modifiers"]:
                    continue

                breaking = False
              
                if len(m["parameters"]) >= 1:
                    method_first_arg = camel_case_to_snake_case(m["parameters"][0]["name"])
                if last_method is None:
                    options_start_at = len(m["parameters"])+1
                    m["options_start_at"] = options_start_at
                    last_method = m
                    method_buildup.append(m)
                else:
                    i = 0
                    if len(last_method["parameters"]) > len(m["parameters"]):
                        can_use_options = False
                        break
                    # for each paramater in this function
                    for i in range(0, len(last_method["parameters"])):
                        if last_method["parameters"][i]["name"] != m["parameters"][i]["name"]:
                            can_use_options = False
                            break

                    m["options_start_at"] = options_start_at
                    method_buildup.append(m)

                    if not can_use_options:
                        breaking = True
                    else:
                        last_method = m
                    i += 1
                n += 1

                if n == len(methods__) and not breaking:
                    method_buildup.append(m)
                    breaking = True
                
                if breaking:
                    method_buildup.append(m)
                    identifier = method_first_arg.split(".")[len(method_first_arg.split("."))-1].lower()
                    method_map[identifier] = method_buildup.copy()
                    method_buildup = []
                    method_first_arg = ""
                    options_start_at = m["options_start_at"]

            for group_name in method_map:
                methods = method_map[group_name]
                if group_name != "":
                    if len(methods) >= 2:
                        new_name = name+"_with_"+group_name.replace("$","").replace("[]","s")
                    else:
                        new_name = name 
                else:
                    new_name = name 
                new_methods[new_name] = {}
                new_methods[new_name]["method"] = methods[len(methods)-1]
                new_methods[new_name]["original_name"] = k

            # hard remove any "_with_" functions that don't actually have a shared function.
            keys = list(map(lambda f: f,new_methods.keys()))
            for key in keys:
                if "_with_" in key:
                    new_new_name = key.split("_with_")[0]
                    if new_new_name not in keys:
                        new_methods[new_new_name] = copy.deepcopy(new_methods[key])
                        new_methods.__delitem__(key)
           
        else:
            new_methods[name] = {}
            new_methods[name]["method"] = method[0]
            new_methods[name]["original_name"] = k

    
    dummy_temp_disable = False
    for k in new_methods:
        method = new_methods[k]

        static = "static" in method["method"]["modifiers"] or is_constructor
        types = method["method"]["parameters"]

        name = k
        if (name.endswith("s") and name != "equals") or "_with_" in name or name == "header":
            if dummy:
                dummy = False
                dummy_temp_disable = True

            
        if name == "to_string":
            if len(types) <= 0:
                has_to_string = True
                has_to_string_is_static = static
                name = "internal_to_string"


        if "options_start_at" in method["method"]:
            options_start_at = method["method"]["options_start_at"]
        else:
            options_start_at = -1

        func_signature = []
        code = []


        if not static:
            func_signature.append({
                "type_name_resolved": "&self",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "qualifiedName": "",
                "type_name_alone": "",
                "generics": [],
                "package_name": "",
                "options_start_at": options_start_at,
                "usage_unsafe": False,
            })
        else:
            func_signature.append({
                "type_name_resolved": "jni: &blackboxmc_general::SharedJNIEnv<'mc>",
                "type_name_lhand": "",
                "is_array": False,
                "is_interface": False,
                "qualifiedName": "",
                "type_name_alone": "",
                "generics": [],
                "package_name": "",
                "options_start_at": options_start_at,
                "usage_unsafe": False,
            })
        should_continue = True
        i = 0
        # make the function signature
        for ty in types:
            
            group = java_type_to_rust(ty["name"], ty["type"]["qualifiedName"],method, i, is_constructor, library)

            if group is None:
                should_continue = False
                break

            func_signature.append(group)
            i += 1

        if not should_continue:
            continue

        # make the inner function.

        obj_call = ""

        prefix = "self.jni_ref()"
        obj_call = "self.jni_object()"

        if is_trait_decl:
            prefix = "self.jni_ref()"
            obj_call = "self.jni_object()"
        if static:
            prefix = "jni"

        # lets parse the types into java.
        n = 0
        types = []

        if is_constructor:
            return_group = java_type_to_rust("", method["method"]["qualifiedName"], method, i, True, library, True)
        else:
            return_group = java_type_to_rust("", method["method"]["returnType"]["qualifiedName"], method, i,True, library, False)

        if return_group is None:
            continue
        
        if not dummy:
            if options_start_at != -1:
                if len(func_signature) > 1:
                    code.append("let mut args = Vec::new();")
                else:
                    code.append("let args = Vec::new();")
                code.append("let mut sig = String::from(\"(\");")
            else:
                code.append("let sig = String::from(\""+java_call_signature_format(func_signature, return_group,is_constructor)+"\");")
        for ty in func_signature:
            if(ty["type_name_lhand"] == "") or in_excluded_classes(ty["qualifiedName"]) or dummy:
                n += 1
                continue
            else:
                typ = code_format(ty, prefix, n, is_array=ty["is_array"], options_start_at=options_start_at, no_sig=False)
                if typ is None:
                    should_continue = False
                    break
                for t in typ:
                    code.append(t)
            if ty["is_array"]:
                types.append("jni::objects::JValueGen::from(val_"+str(n)+".l()?)")
            else:
                types.append("jni::objects::JValueGen::from(val_"+str(n)+")")
            n += 1

        if not dummy:
            if options_start_at != -1:
                if not is_constructor:
                    code.append("sig += \")"+java_letter_from_rust(return_group["qualifiedName"],return_group["is_array"])+"\";")
                else:
                    code.append("sig += \")V\";")
        if(not should_continue):
            continue



        if("?" in return_group["type_name_resolved"]):
            continue

        if in_excluded_classes(return_group["qualifiedName"]):
            continue

        if return_group["type_name_alone"] in omitted_classes:
            continue

        for generic in return_group["generics"]:
            parts = generic["type_name_resolved"].split("::")
            if parts[len(parts)-1].replace("_","$") in omitted_classes:
                should_continue = False
                break

        if(not should_continue):
            continue

        nullable = False
        impl_signature_canidate = []
        if "docString" in method["method"]:
            comment_raw = method["method"]["docString"].replace("\n","")
        else:
            comment_raw = ""
        if "annotations" in method["method"]:
            (impl_signature_canidate,nullable,new_comment) = parse_annotations(method["method"]["annotations"],comment_raw)
            if new_comment is not None:
                method["method"]["docString"] = new_comment


        func_signature_resolved = ""
        func_signature_resolved_parts = []
        func_signature_resolved_parts_names_only = []
        generic_letters = []
        generic_args = []
        j = 0

        for ty in func_signature:
            if ty["usage_unsafe"]:
                usage_unsafe = True
            group = func_signature_format(ty,j,False,options_start_at)

            if group["result"] != "":
                func_signature_resolved_parts.append(group["result"])
                func_signature_resolved_parts_names_only.append(group["result_name_only"])
                j += 1

            generic_letters += group["generic_letters"]
            generic_args += group["generic_args"]

        func_signature_resolved = ",".join(func_signature_resolved_parts)
        func_signature_resolved_raw = ",".join(list(map(lambda f: f.replace("&self","this: "+og_name).replace(": ",": *mut ").replace("<'mc>","<'static>"),func_signature_resolved_parts)))
        
        func_signature_resolved_names_only = ",".join(list(map(lambda f: f, filter(lambda f: f != "&self",func_signature_resolved_parts_names_only))))
        func_signature_resolved_raw_names_only = ",".join(list(map(lambda f: "*"+f+".as_ref().unwrap()", filter(lambda f: f != "&self",func_signature_resolved_parts_names_only))))

        # execute the function.
        m = return_format(return_group, prefix, static, method, obj_call, val, types, is_trait,options_start_at, is_constructor,nullable,library)
        if m is None:
            continue

        if dummy:
            if name == "of" and "$" in og_name:
                continue
            val_group = java_type_to_rust("", val["qualifiedName"], None, 0, True, library, False)
            if val_group is None:
                continue
            if in_excluded_classes(val_group["qualifiedName"]):
                continue
            if val_group["type_name_alone"] in omitted_classes:
                continue
            if static:
                tyname = val_group["type_name_resolved"]
                if(val_group["qualifiedName"] in enums):
                    tyname += "Struct"
                code.append(tyname+"::"+name+"("+func_signature_resolved_names_only+")")
            else:
                parsed_name = parsed_names[len(parsed_names)-1]
                # we do an unsafe clone here but justify it because it's only to execute the method. said unsafe clone
                # will go out of scope after this method is done.
                if parsed_name == "jni::objects::JObject":
                    code.append("let temp_clone = unsafe {jni::objects::JObject::from_raw(self.1.clone())};")
                else:
                    code.append("let temp_clone = "+val_group["type_name_resolved"]+"::from_raw(&self.0, unsafe {jni::objects::JObject::from_raw(self.1.clone())})?;")
                code.append("let real: "+val_group["type_name_resolved"]+" = temp_clone.into();")
                code.append("real."+name+"("+func_signature_resolved_names_only+")")
        else:
            impl_signature += impl_signature_canidate
            code.append(m)


        return_ty = func_signature_format(return_group,j,True)

        generic_letters_str = ""
        if len(generic_letters) >= 1:
            generic_letters_str = "<"+",".join(generic_letters)+">"
        generic_args_str = ""
        if len(generic_args) >= 1:
            generic_args_str = " where "+",".join(generic_args)


        
        if "docString" in method["method"]:
            impl_signature.append(format_comment(method["method"]["docString"]))
        if name.startswith("internal_"):
            impl_signature.append("#[doc(hidden)]")
        impl_signature.append(
            "\tpub fn "+name+generic_letters_str+"("+func_signature_resolved+") "
        )


        # C exports
        extern_rs = crate_dir+os.path.sep+"extern.rs"
        if extern_rs not in file_cache:
            file_cache[extern_rs] = []
        file_cache[extern_rs].append("""
extern \"C"\" fn """+name+"""("""+func_signature_resolved+""");""")

        if nullable:
            return_ty["result"] = "Option<"+return_ty["result"]+">"

        impl_signature.append("-> Result<"+return_ty["result"]+", Box<dyn std::error::Error>>")

        impl_signature.append(generic_args_str)

        if is_trait and not is_trait_decl:
            impl_signature.append(";")
        else:
            impl_signature.append("{"+"\n".join(code)+"}")

        raw_return = "*mut "+return_ty["result"].replace("<'mc>","<'static>")
        # c binding.
        #if static:
        #    extern_signature.append("#[no_mangle]\npub extern fn "+camel_case_to_snake_case(og_name)+"_"+name+"("+func_signature_resolved_raw+") -> Result<"+raw_return+", Box<dyn std::error::Error>> ")
        #    extern_signature.append("{\n\tunsafe {\n\t\tOk(Box::leak(Box::new("+og_name+"::"+name+"("+func_signature_resolved_names_only+")?)) as "+raw_return+")\n\t}\n}")
        #else:
        #    extern_signature.append("#[no_mangle]\npub extern fn "+camel_case_to_snake_case(og_name)+"_"+name+"("+func_signature_resolved_raw+") -> Result<"+raw_return+", Box<dyn std::error::Error>> ")
        #    extern_signature.append("{\n\tunsafe {\n\t\tOk(Box::leak(Box::new(this.as_ref().unwrap()."+name+"("+func_signature_resolved_names_only+")?)) as "+raw_return+")\n\t}\n}")

        if dummy_temp_disable:
            dummy = True
            
    for impl in impl_signature:
        file_cache[mod_path].append(impl)

    if not is_constructor:
        if not dummy:
            method_names = list(map(lambda k: new_methods[k]["original_name"],new_methods))
            gen_dummy(val,method_names,mod_path,is_trait,is_trait_decl,parsed_names,library,False)

    return {
        "has_to_string": has_to_string,
        "has_to_string_is_static": has_to_string_is_static,
        "extern_signature": extern_signature,
    }

def gen_dummy(val,method_names,mod_path,is_trait,is_trait_decl,parsed_names,library,on_super_class):
    v = copy.deepcopy(val)

    if on_super_class:
        if "methods" in v:
           
            methods = list(filter(lambda f: f["name"] not in method_names, v["methods"]))
            v["methods"] = methods

            parse_methods(library,v["name"],v,mod_path,is_trait,is_trait_decl,False,True,parsed_names)
            method_names += list(map(lambda f: f["name"], v["methods"]))
    else:
        if "superclass" in v:
            gen_dummy_super_class(v["superclass"],method_names,mod_path,is_trait,is_trait_decl,parsed_names,library)
def gen_dummy_super_class(val,method_names,mod_path,is_trait,is_trait_decl,parsed_names,library):
    if val["qualifiedName"] not in libraries[library]:
        return

    val = copy.deepcopy(libraries[library][val["qualifiedName"]])
    val["methods"] = list(filter(lambda f: f["name"] not in method_names, val["methods"]))
 
    file_cache[mod_path].append("// SUPER CLASS: "+str(val["qualifiedName"])+" ( "+str(method_names)+")")

    packageName = ".".join(list(filter(lambda f: f.lower() == f,val["qualifiedName"].split("."))))
    new_parsed_name = packageName+"."+val["name"].replace("$", "").replace("-", "_")

    if in_excluded_classes(new_parsed_name):
        return
    
    crate_name = ".".join(filter(lambda f: f.lower() == f, new_parsed_name.split(".")))

    while crate_name not in library_resolves and crate_name != "":
        parts = crate_name.split(".")
        parts.pop()
        crate_name = ".".join(parts)

    if (library_name_format(crate_name,library) == library_name_format("java.util",library)) or (library_name_format(crate_name,library) == library_name_format("java.lang",library)):
        class_name = "".join(filter(lambda f: f[0].upper() == f[0], new_parsed_name.split(".")))
        new_parsed_name = new_parsed_name.replace(class_name, "Java"+class_name)

    to_replace =  library_name_format(crate_name,library)

    new_parsed_name = new_parsed_name.replace(
        crate_name, to_replace).replace(".", "::").replace("-", "_").replace("$", "")

    if "JavaObject" in new_parsed_name or "JavaEnum" in new_parsed_name:
        return
    gen_dummy(val,method_names,mod_path,is_trait,is_trait_decl,parsed_names+[new_parsed_name],library,True)
    method_names += list(map(lambda f: f["name"], val["methods"]))

def format_comment(comment):
    final_comment = ""
    parts = comment.replace("<br>"," \n \n").replace("<p>","").replace("</p>","").split("\n")
    for comm in parts:
        while "  " in comm or "\n" in comm or "\r" in comm:
            comm = comm.replace("  ","").replace("\n","").replace("\r","")
        if comm != "":
            if comm[0] == " ":
                comm = comm[1:]
            final_comment +=  "/// "+comm+"\n"
    if final_comment.endswith("\n"):
        final_comment = final_comment[0:len(final_comment)-1]
    return final_comment

def parse_classes(library, val, classes):
    if "modifiers" in val:
        modifiers = val["modifiers"]
        if ("public" not in modifiers):
            omitted_classes.append(val["name"])
            return
    packageName = ".".join(list(filter(lambda f: f.lower() == f,val["qualifiedName"].split("."))))
    dir = packageName.replace(".", os.sep)
    mod_path = dir+os.sep+"mod.rs"
    name = val["name"].replace(".", "").replace("-", "_")

    if name == "Result" or name == "Option":
        name = "Spigot"+name
        val["name"] = name

    full_name = packageName+"."+val["name"]
    if library.startswith("java"):
        name = "Java"+name
    if in_excluded_classes(full_name):
        print("excluding "+full_name)
        return

    if mod_path in parsed_classes:
        if full_name in parsed_classes[mod_path]:
            return
        else:
            parsed_classes[mod_path].append(full_name)
    else:
        parsed_classes[mod_path] = [full_name]

    if mod_path not in file_cache:
        file_cache[mod_path] = ["#![allow(deprecated)]\nuse blackboxmc_general::JNIRaw;\nuse blackboxmc_general::JNIInstantiatable;\nuse color_eyre::eyre::Result;"]

    if (name == ""):
        return

    if "docString" in val:
        file_cache[mod_path].append(format_comment(val["docString"]))

    if "superclass" in val:
        if "qualifiedName" in val["superclass"]:
            if val["superclass"]["qualifiedName"] == "java.lang.Enum":
                val["isEnum"] = True
                val["fields"] = val["constants"]
            else:
                meth = list(filter(lambda f: f["name"] == "values",val["methods"]))
                if len(meth) >= 1 and len(val["fields"]) >= 1:
                    val["isEnum"] = True
                else:
                    val["isEnum"] = False
        else:
            val["isEnum"] = False
    else:
        val["isEnum"] = False

    if "modifiers" in val:
        if "interface" in val["modifiers"]:
            val["isInterface"] = True
        else:
            val["isInterface"] = False
    else:
        val["isInterface"] = False
        
    if val["isEnum"]:  # enum generation
        if "values" not in val:
            val["values"] = []

        val["values"] += list(map(lambda f: f["name"],val["fields"]))
        
        file_cache[mod_path].append(
            "pub enum "+name+"<'mc> {")


        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            if "annotations" in val:
                if v in val["annotations"]:
                    file_cache[mod_path] += parse_annotations(val["annotations"][v],"")[0]
            file_cache[mod_path].append("\t"+val_proper+" {inner: "+name+"Struct<'mc>},")


        file_cache[mod_path].append("}")

        variants = []
        for v in val["values"]:
            val_proper = snake_case_to_camel_case(v)
            val_proper = val_proper[0].upper() + val_proper[1:]
            if val_proper.lower() in reserved_words:
                val_proper = "Variant"+val_proper
            variants.append((v,val_proper))

        # DISPLAY IMPL

        file_cache[mod_path].append("impl<'mc> std::fmt::Display for "+name+"<'mc> {")
        file_cache[mod_path].append("   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {")
        file_cache[mod_path].append("       match self {")
        for (v,val_proper) in variants:
            file_cache[mod_path].append("           "+name+"::"+val_proper+" { .. } => f.write_str(\""+v+"\"),")
        file_cache[mod_path].append("       }")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("impl<'mc> std::ops::Deref for "+name+"<'mc> {")
        file_cache[mod_path].append("   type Target = "+name+"Struct<'mc>;")
        file_cache[mod_path].append("   fn deref(&self) -> &<"+name+"<'mc> as std::ops::Deref>::Target {")
        file_cache[mod_path].append("       match self {")
        for (v,val_proper) in variants:
            file_cache[mod_path].append("           "+name+"::"+val_proper+" { inner } => inner,")
        file_cache[mod_path].append("       }")
        file_cache[mod_path].append("   }")
        file_cache[mod_path].append("}")

        file_cache[mod_path].append("""
        impl<'mc> """+name+"""<'mc> {
            pub fn value_of(
                env: &blackboxmc_general::SharedJNIEnv<'mc>,
                arg0: impl Into<String>,
            ) -> Result<"""+name+"""<'mc>, Box<dyn std::error::Error>> {
                let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
                let cls = env.find_class(\""""+full_name.replace(".","/")+"""\");
                let cls = env.translate_error_with_class(cls)?;
                let res = env.call_static_method(
                    cls,
                    "valueOf",
                    "(Ljava/lang/String;)L"""+full_name.replace(".","/")+""";",
                    vec![jni::objects::JValueGen::from(val_1)],
                );
                let res = env.translate_error(res)?;
                let obj = res.l()?;
                let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
                let variant = env.translate_error(variant)?;
                let variant_str = env
                    .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                    .to_string_lossy()
                    .to_string();
                match variant_str.as_str() {
                    """)
        for (v,val_proper) in variants:
            file_cache[mod_path].append("\""+v+"\" => Ok("+name+"::"+val_proper+" { inner: "+name+"Struct::from_raw(env,obj)?}),")

        file_cache[mod_path].append("""
                    _ => Err(eyre::eyre!(\"String gaven for variant was invalid\").into())
                }
            }
        }
        """)

        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"Struct<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);")

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library, cl, classes)

        gen_jniraw_impl(name, True, mod_path, full_name, variants)

        file_cache[mod_path].append("impl<'mc> "+name+"Struct<'mc> {")

        has_to_string = False
        has_to_string_is_static = False
        extern = []
        if "methods" in val:
            grp = parse_methods(library, name,val,mod_path,False,False,False,False,[name+"Struct"])
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]

        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")

        for ex in extern:
            file_cache[mod_path].append(ex)

    elif val["isInterface"]: # interface generation
     
        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )

        gen_jniraw_impl(name, False, mod_path, full_name, None)

        file_cache[mod_path].append(
            "impl<'mc> "+name+"<'mc> {")

        has_to_string = False
        has_to_string_is_static = False

        extern = []
        if "methods" in val:
            grp = parse_methods(library,name,val,mod_path,True,True,False,False,[name])
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]


        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")

        for ex in extern:
            file_cache[mod_path].append(ex)

        if has_to_string:
            file_cache[mod_path].append(gen_to_string_func(name,has_to_string_is_static))
    else:  # struct generation
        file_cache[mod_path].append(
            "#[repr(C)]\npub struct "+name+"<'mc>(pub(crate) blackboxmc_general::SharedJNIEnv<'mc>, pub(crate) jni::objects::JObject<'mc>);"
        )

        if "classes" in val:
            for cl in val["classes"]:
                parse_classes(library,cl, classes)

        gen_jniraw_impl(name, False, mod_path, full_name, None)

        file_cache[mod_path].append("impl<'mc> "+name+"<'mc> {")

        if "constructors" in val:
            grp = parse_methods(library, name,val,mod_path,False,False,True,False,[name])

        has_to_string = False
        has_to_string_is_static = False

        extern = []
        if "methods" in val:
            grp = parse_methods(library,name,val,mod_path,False,False,False,False,[name])
            has_to_string = grp["has_to_string"]
            has_to_string_is_static = grp["has_to_string_is_static"]
            extern += grp["extern_signature"]

        gen_instance_of_func( mod_path)

        file_cache[mod_path].append("}")
        for ex in extern:
            file_cache[mod_path].append(ex)

        if has_to_string:
            file_cache[mod_path].append(gen_to_string_func(name,has_to_string_is_static))

    if not val["isEnum"]:
        if "interfaces" in val:
            for inter in val["interfaces"]:
                parse_into_impl(inter,name,mod_path)
        if "superclass" in val:
            super_class = val["superclass"]
            parse_into_impl(super_class,name,mod_path)
    
    
    # make a blank, "Class" struct that can be passed as a function.
    #file_cache[mod_path].append("""
    #    #[repr(C)]
    #    pub struct """+name+"""Class;
    #    impl blackboxmc_general::JNIProvidesClassName for """+name+"""Class {
    #        fn class_name(&self) -> &str {
    #            \""""+packageName.replace(".","/")+"/"+val["name"]+"""\"
    #        }
    #    }
    #""")


def parse_into_impl(val,name,mod_path):
    packageName = ".".join(list(filter(lambda f: f.lower() == f,val["qualifiedName"].split("."))))

    if packageName.startswith("java"):
        if not packageName.startswith("java.util"):
            return
    
    val_resolved = java_type_to_rust("", val["qualifiedName"], None, 0, True, library, False)

    if val_resolved is None:
        return
    if in_excluded_classes(val_resolved["qualifiedName"]):
        return
    if val_resolved["type_name_alone"] in omitted_classes:
        return

    # we want to check if the generics are the same.
    if ".".join(val_resolved["package_name"].split(".")[0:2]) in libraries:
        temp_pkg = libraries[".".join(val_resolved["package_name"].split(".")[0:2])]

        tyalone = "".join(filter(lambda f: f[0].isupper(), val_resolved["qualifiedName"].split(".")))
        if tyalone in temp_pkg:
            temp_cls = temp_pkg[tyalone]

    if "impl<'mc> Into<"+val_resolved["type_name_resolved"]+"<'mc>> for "+name+"<'mc>{\n" in file_cache[mod_path]:
        return

    file_cache[mod_path].append("impl<'mc> Into<"+val_resolved["type_name_resolved"]+"<'mc>> for "+name+"<'mc>{\n")
    file_cache[mod_path].append("fn into(self) -> "+val_resolved["type_name_resolved"]+"<'mc> {\n")
    if val_resolved["type_name_resolved"] == "jni::objects::JObject":
        file_cache[mod_path].append("self.1")
    else:
        file_cache[mod_path].append(val_resolved["type_name_resolved"]+"::from_raw(&self.jni_ref(), self.1).expect(\"Error converting "+name+" into "+val_resolved["type_name_resolved"]+"\")\n")
    file_cache[mod_path].append("   }\n"+
                            "}")

dep_comment_regex = "<div class=\"deprecation-comment\">(.*?)</div>"

def add_deprecated(arr,comment):
    if "deprecation-comment" in comment:
        html_parsed_with_regex_lmao = re.search(dep_comment_regex,comment)
        if html_parsed_with_regex_lmao is not None:
            comm = html_parsed_with_regex_lmao.group(1)
            while "  " in comm:
                comm = comm.replace("  "," ")
            if comm.startswith(" "):
                comm = comm[1:]
            if comm.endswith(" "):
                comm = comm[:len(comm)]
            comment = comment.replace(comm,"") # idk why the code below doesn't catch this but ok
            arr.append("#[deprecated = \""+comm.replace("\"","'")+"\"]")
        else:
            arr.append("#[deprecated]")
    else:
        arr.append("#[deprecated]")
    comment = re.sub(dep_comment_regex,"",comment)
    comment = re.sub("<span class=\"deprecated-label\">(.*?)</span>","",comment)
    return comment

def parse_annotations(annotations,comment):
    strings = []
    nullable = False
    new_comment = None
    for annotation in annotations:
        match(annotation["typeName"]):
            case "Deprecated":
                new_comment = add_deprecated(strings,comment)
            case "Experimental":
                nop = 0
            case "Nullable":
                nullable = True
    return (strings,nullable,new_comment)
# what we first want to do is collect any interfaces.
for package in libraries:
    classes = libraries[package]
    
    for clas in classes:

        packageName = clas["qualifiedName"].replace("::",".")
        if "interface" in clas["modifiers"]:
            interface_names.append(packageName)
        
        if "superclass" in clas:
            if clas["superclass"]["qualifiedName"] == "java.lang.Enum" or "org.bukkit.Keyed" in list(map(lambda f: f["qualifiedName"], clas["interfaces"])):
                enums.append(packageName)
            else:
                meth = list(filter(lambda f: f["name"] == "clasues",clas["methods"]))
                if len(meth) >= 1 and len(clas["fields"]) >= 1:
                    enums.append(packageName)
        if "classes" in clas:
            for cla in clas["classes"]:
                if "superclass" in cla:
                    if cla["superclass"]["qualifiedName"] == "java.lang.Enum" or "org.bukkit.Keyed" in list(map(lambda f: f["qualifiedName"], cla["interfaces"])):
                        enums.append(packageName)
                    else:
                        meth = list(filter(lambda f: f["name"] == "clasues",cla["methods"]))
                        if len(meth) >= 1 and len(cla["fields"]) >= 1:
                            enums.append(packageName)

for library in libraries:
    packages = libraries[library]

    crate_dir = ""

    if library in library_resolves:
        res = library_resolves[library]
        if "src" in res:
            crate_dir = res + os.sep
        else:
            crate_dir = os.path.join(res, "src")
    else:
        print("Unhandled library "+library +
            ". All libraries must have corresponding rust crates.")
        exit(0)

    root = library_name_no_extra_paths(crate_dir)
    if "src" not in root:
        root += os.sep + "src"

    # delete and recreate the appropriate directory if it's the first time writing to it
    if pathlib.Path(crate_dir).exists():
        shutil.rmtree(crate_dir)
        path = library.replace(".", os.sep)
        pathlib.Path(crate_dir+os.sep +
                    path).mkdir(parents=True, exist_ok=True)


    # make the appropriate directory if it doesn't already exist.
    path = library.replace(".", os.sep)
    pathlib.Path(crate_dir+os.sep +
                path).mkdir(parents=True, exist_ok=True)
    
    p = {}
    for pkg in packages:
        p[pkg["qualifiedName"]] = pkg
    packages = p
    libraries[library] = packages
    for clas in packages:
        added = []
        clas = packages[clas]
        parse_classes(library, clas, classes)

    for k in file_cache:
        val = file_cache[k]
        ok = k.split(os.sep)
        ok.pop()
        ok = os.sep.join(ok)
        pathlib.Path(crate_dir+os.sep +
                    ok).mkdir(parents=True, exist_ok=True)
        with open(crate_dir+os.sep+k, "w") as file:
            file.truncate(0)
            for v in val:
                file.write(v+"\n")
            file.close()

    file_cache = {}

    # move everything in org/bukkit into root
    parts = library.split(".")
    dir = crate_dir + os.sep + os.sep.join([parts[0], parts[1]])
    shutil.copytree(dir, crate_dir, dirs_exist_ok=True)
    shutil.rmtree(crate_dir + os.sep + parts[0])

    mod_rs = crate_dir+os.sep+"lib.rs"

    if mod_rs not in file_cache:
        file_cache[mod_rs] = []

    with open(mod_rs, "a+") as file:
        for v in file_cache[mod_rs]:
            file.write(v+"\n")
        file.close()

    file_cache = {}


    if root not in filled_once:
        mod_rs_folder_populate(root)
        os.rename(root+os.sep+"mod.rs", root+os.sep+"lib.rs")
    filled_once.append(root)

# inject any manually written code.
for filename in os.listdir("additions"):
    if not filename.endswith(".rs"):
        continue
    with open(os.path.join("additions", filename), "r") as f:
        lines = f.readlines()
        opening_line = lines[0]
        if(not opening_line.startswith("//")):
            continue
        # if it has "#", then that's a struct that this is appended to.
        if("#" in opening_line):
            parts = opening_line.split("#")
            opening_line = parts[0]
            append_to_struct = parts[1].replace("\n","").replace("\r","")
        else:
            append_to_struct = None
        filename2 = opening_line.replace("// ","")
        parent_dir = filename2.replace(os.sep+"mod.rs", "")
        if not os.path.exists(parent_dir):
            os.makedirs(parent_dir)
        with open(filename2, "a+") as f2:
            lines = "".join(lines[1:])
            if append_to_struct is not None:
                f2.seek(0)
                newlines = f2.readlines()
                at = int(newlines.index("impl<'mc> "+append_to_struct+"<'mc> {\n"))
                newlines.insert(at+1, lines)
                f2.truncate(0)
                f2.write("".join(newlines))
            else:
                f2.write(lines)
            f2.close()
        f.close()

for i in range(0,4):
    wildcard = "*/*" * i
    for library in library_resolves:
        resolved = library_resolves[library]
        if "src" not in resolved:
            resolved += "/src"
        os.system("rustfmt  ./"+resolved+"/*"+wildcard+".rs")

os.system("cargo fix --allow-dirty --allow-staged --broken-code --jobs "+str(multiprocessing.cpu_count()))


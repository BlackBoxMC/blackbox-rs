#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
#[repr(C)]
pub struct YamlConstructor<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for YamlConstructor<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for YamlConstructor<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate YamlConstructor from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/YamlConstructor")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlConstructor object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> YamlConstructor<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct YamlConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for YamlConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for YamlConfiguration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate YamlConfiguration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/YamlConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> YamlConfiguration<'mc> {
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::configuration::file::YamlConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()V");
        let cls = jni.find_class("org/bukkit/configuration/file/YamlConfiguration");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), vec![]);
        let res = jni.translate_error_no_gen(res)?;
        crate::configuration::file::YamlConfiguration::from_raw(&jni, res)
    }

    pub fn save_to_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/lang/String;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "saveToString", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }

    pub fn load_from_string(
        &self,
        contents: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Ljava/lang/String;)V");
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(contents.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "loadFromString",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn options(
        &self,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/file/YamlConfigurationOptions;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "options", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.configuration.file.FileConfiguration ( ['saveToString', 'loadFromString', 'options', 'loadConfiguration'])
    /// Saves this {@link FileConfiguration} to the specified location.
    ///
    /// If the file does not exist, it will be created. If already exists, it
    /// will be overwritten. If it cannot be overwritten or created, an
    /// exception will be thrown.
    ///
    /// This method will save using the system default encoding, or possibly
    /// using UTF8.
    pub fn save(&self, file: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::configuration::file::FileConfiguration::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::configuration::file::FileConfiguration = temp_clone.into();
        real.save(file)
    }
    /// Loads this {@link FileConfiguration} from the specified location.
    ///
    /// All the values contained within this configuration will be removed,
    /// leaving only settings and defaults, and the new values will be loaded
    /// from the given file.
    ///
    /// If the file cannot be loaded for any reason, an exception will be
    /// thrown.
    pub fn load(&self, file: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone =
            crate::configuration::file::FileConfiguration::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::configuration::file::FileConfiguration = temp_clone.into();
        real.load(file)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::file::FileConfiguration<'mc>> for YamlConfiguration<'mc> {
    fn into(self) -> crate::configuration::file::FileConfiguration<'mc> {
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting YamlConfiguration into crate::configuration::file::FileConfiguration",
        )
    }
}
#[repr(C)]
pub struct FileConfiguration<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FileConfiguration<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FileConfiguration<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate FileConfiguration from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/FileConfiguration")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FileConfiguration object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FileConfiguration<'mc> {
    /// Creates an empty {@link FileConfiguration} using the specified {@link
    /// Configuration} as a source for all default values.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        defaults: std::option::Option<impl Into<crate::configuration::Configuration<'mc>>>,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = defaults {
            sig += "Lorg/bukkit/configuration/Configuration;";
            let val_1 = jni::objects::JValueGen::Object(unsafe {
                jni::objects::JObject::from_raw(a.into().jni_object().clone())
            });
            args.push(val_1);
        }
        sig += ")V";
        let cls = jni.find_class("org/bukkit/configuration/file/FileConfiguration");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(cls, sig.as_str(), args);
        let res = jni.translate_error_no_gen(res)?;
        crate::configuration::file::FileConfiguration::from_raw(&jni, res)
    }
    /// Saves this {@link FileConfiguration} to the specified location.
    ///
    /// If the file does not exist, it will be created. If already exists, it
    /// will be overwritten. If it cannot be overwritten or created, an
    /// exception will be thrown.
    ///
    /// This method will save using the system default encoding, or possibly
    /// using UTF8.
    pub fn save(&self, file: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(file.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "save", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Loads this {@link FileConfiguration} from the specified location.
    ///
    /// All the values contained within this configuration will be removed,
    /// leaving only settings and defaults, and the new values will be loaded
    /// from the given file.
    ///
    /// If the file cannot be loaded for any reason, an exception will be
    /// thrown.
    pub fn load(&self, file: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Ljava/lang/String;";
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(file.into())?,
        ));
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "load", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn options(
        &self,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/file/FileConfigurationOptions;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "options", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.configuration.MemoryConfiguration ( ['save', 'load', 'options'])

    pub fn add_default(
        &self,
        path: impl Into<String>,
        value: jni::objects::JObject<'mc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let temp_clone = crate::configuration::MemoryConfiguration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemoryConfiguration = temp_clone.into();
        real.add_default(path, value)
    }

    pub fn add_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        sig += "Lorg/bukkit/configuration/Configuration;";
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        args.push(val_1);
        sig += ")V";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "addDefaults", sig.as_str(), args);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn set_defaults(
        &self,
        defaults: impl Into<crate::configuration::Configuration<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/configuration/Configuration;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(defaults.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }

    pub fn defaults(
        &self,
    ) -> Result<Option<crate::configuration::Configuration<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/configuration/Configuration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getDefaults", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::configuration::Configuration::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn parent(
        &self,
    ) -> Result<Option<crate::configuration::ConfigurationSection<'mc>>, Box<dyn std::error::Error>>
    {
        let temp_clone = crate::configuration::MemoryConfiguration::from_raw(&self.0, unsafe {
            jni::objects::JObject::from_raw(self.1.clone())
        })?;
        let real: crate::configuration::MemoryConfiguration = temp_clone.into();
        real.parent()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::MemoryConfiguration<'mc>> for FileConfiguration<'mc> {
    fn into(self) -> crate::configuration::MemoryConfiguration<'mc> {
        crate::configuration::MemoryConfiguration::from_raw(&self.jni_ref(), self.1).expect(
            "Error converting FileConfiguration into crate::configuration::MemoryConfiguration",
        )
    }
}
#[repr(C)]
pub struct FileConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for FileConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for FileConfigurationOptions<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate FileConfigurationOptions from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/file/FileConfigurationOptions",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a FileConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> FileConfigurationOptions<'mc> {
    pub fn configuration(
        &self,
    ) -> Result<crate::configuration::file::FileConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/file/FileConfiguration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "configuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_defaults(
        &self,
        value: bool,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Z)Lorg/bukkit/configuration/file/FileConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn path_separator(
        &self,
        value: u16,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(C)Lorg/bukkit/configuration/file/FileConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Char(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the header that will be applied to the top of the saved output.
    ///
    /// This header will be commented out and applied directly at the top of
    /// the generated output of the {@link FileConfiguration}. It is not
    /// required to include a newline at the end of the header as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_header(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getHeader", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    #[deprecated]

    pub fn header(
        &self,
        value: std::option::Option<impl Into<String>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "Ljava/lang/String;";
            let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
                self.jni_ref().new_string(a.into())?,
            ));
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/file/FileConfigurationOptions;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "header", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets the header that will be applied to the top of the saved output.
    ///
    /// This header will be commented out and applied directly at the top of
    /// the generated output of the {@link FileConfiguration}. It is not
    /// required to include a newline at the end of the header as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn set_header(
        &self,
        value: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
        );
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in value {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Gets the footer that will be applied to the bottom of the saved output.
    ///
    /// This footer will be commented out and applied directly at the bottom of
    /// the generated output of the {@link FileConfiguration}. It is not required
    /// to include a newline at the beginning of the footer as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn footer(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let sig = String::from("()Ljava/util/List;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getFooter", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let list = blackboxmc_java::util::JavaList::from_raw(&self.0, res.l()?)?;
        let iter = list.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(
                self.jni_ref()
                    .get_string(unsafe { &jni::objects::JString::from_raw(*obj) })?
                    .to_string_lossy()
                    .to_string(),
            );
        }
        Ok(new_vec)
    }
    /// Sets the footer that will be applied to the bottom of the saved output.
    ///
    /// This footer will be commented out and applied directly at the bottom of
    /// the generated output of the {@link FileConfiguration}. It is not required
    /// to include a newline at the beginning of the footer as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn set_footer(
        &self,
        value: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/FileConfigurationOptions;",
        );
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in value {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFooter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets whether or not comments should be loaded and saved.
    ///
    /// Defaults to true.
    pub fn parse_comments(
        &self,
        value: std::option::Option<bool>,
    ) -> Result<crate::configuration::MemoryConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/MemoryConfigurationOptions;";
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "parseComments", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn copy_header(
        &self,
        value: std::option::Option<bool>,
    ) -> Result<crate::configuration::file::FileConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "Z";
            let val_1 = jni::objects::JValueGen::Bool(a.into());
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/file/FileConfigurationOptions;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "copyHeader", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.configuration.MemoryConfigurationOptions ( ['configuration', 'copyDefaults', 'pathSeparator', 'getHeader', 'header', 'setHeader', 'getFooter', 'setFooter', 'parseComments', 'copyHeader'])

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::MemoryConfigurationOptions<'mc>>
    for FileConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::MemoryConfigurationOptions<'mc> {
        crate::configuration::MemoryConfigurationOptions::from_raw(&self.jni_ref(), self.1).expect("Error converting FileConfigurationOptions into crate::configuration::MemoryConfigurationOptions")
    }
}
#[repr(C)]
pub struct YamlRepresenter<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for YamlRepresenter<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for YamlRepresenter<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate YamlRepresenter from null object.").into(),
            );
        }
        let (valid, name) =
            env.validate_name(&obj, "org/bukkit/configuration/file/YamlRepresenter")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlRepresenter object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> YamlRepresenter<'mc> {
    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct YamlConfigurationOptions<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for YamlConfigurationOptions<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for YamlConfigurationOptions<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!(
                "Tried to instantiate YamlConfigurationOptions from null object."
            )
            .into());
        }
        let (valid, name) = env.validate_name(
            &obj,
            "org/bukkit/configuration/file/YamlConfigurationOptions",
        )?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a YamlConfigurationOptions object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> YamlConfigurationOptions<'mc> {
    pub fn configuration(
        &self,
    ) -> Result<crate::configuration::file::YamlConfiguration<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("()Lorg/bukkit/configuration/file/YamlConfiguration;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "configuration", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfiguration::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn copy_defaults(
        &self,
        value: bool,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Z)Lorg/bukkit/configuration/file/YamlConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyDefaults",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn path_separator(
        &self,
        value: u16,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(C)Lorg/bukkit/configuration/file/YamlConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Char(value);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "pathSeparator",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_header(
        &self,
        value: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
        );
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in value {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setHeader",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn header(
        &self,
        value: impl Into<String>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/lang/String;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
        );
        let val_1 = jni::objects::JValueGen::Object(jni::objects::JObject::from(
            self.jni_ref().new_string(value.into())?,
        ));
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "header",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn set_footer(
        &self,
        value: Vec<jni::objects::JObject<'mc>>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from(
            "(Ljava/util/List;)Lorg/bukkit/configuration/file/YamlConfigurationOptions;",
        );
        let raw_val_1 = self
            .jni_ref()
            .new_object("java/util/ArrayList", "()V", vec![])?;
        for v in value {
            let map_val_0 = jni::objects::JValueGen::Object(v);
            self.jni_ref().call_method(
                &raw_val_1,
                "add",
                "(Ljava/lang/Object;)Z",
                vec![jni::objects::JValueGen::from(map_val_0)],
            )?;
        }
        let val_1 = jni::objects::JValueGen::Object(raw_val_1);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setFooter",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn parse_comments(
        &self,
        value: bool,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Z)Lorg/bukkit/configuration/file/YamlConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "parseComments",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]

    pub fn copy_header(
        &self,
        value: bool,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let sig = String::from("(Z)Lorg/bukkit/configuration/file/YamlConfigurationOptions;");
        let val_1 = jni::objects::JValueGen::Bool(value.into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "copyHeader",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets how much spaces should be used to indent each line.
    ///
    /// The minimum value this may be is 2, and the maximum is 9.
    pub fn indent(
        &self,
        value: std::option::Option<i32>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/file/YamlConfigurationOptions;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "indent", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Sets how long a line can be, before it gets split.
    pub fn width(
        &self,
        value: std::option::Option<i32>,
    ) -> Result<crate::configuration::file::YamlConfigurationOptions<'mc>, Box<dyn std::error::Error>>
    {
        let mut args = Vec::new();
        let mut sig = String::from("(");
        if let Some(a) = value {
            sig += "I";
            let val_1 = jni::objects::JValueGen::Int(a);
            args.push(val_1);
        }
        sig += ")Lorg/bukkit/configuration/file/YamlConfigurationOptions;";
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "width", sig.as_str(), args);
        let res = self.jni_ref().translate_error(res)?;
        crate::configuration::file::YamlConfigurationOptions::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    // SUPER CLASS: org.bukkit.configuration.file.FileConfigurationOptions ( ['configuration', 'copyDefaults', 'pathSeparator', 'setHeader', 'header', 'setFooter', 'parseComments', 'copyHeader', 'indent', 'width'])
    /// Gets the header that will be applied to the top of the saved output.
    ///
    /// This header will be commented out and applied directly at the top of
    /// the generated output of the {@link FileConfiguration}. It is not
    /// required to include a newline at the end of the header as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn get_header(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::configuration::file::FileConfigurationOptions::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::configuration::file::FileConfigurationOptions = temp_clone.into();
        real.get_header()
    }
    /// Gets the footer that will be applied to the bottom of the saved output.
    ///
    /// This footer will be commented out and applied directly at the bottom of
    /// the generated output of the {@link FileConfiguration}. It is not required
    /// to include a newline at the beginning of the footer as it will
    /// automatically be applied, but you may include one if you wish for extra
    /// spacing.
    ///
    /// If no comments exist, an empty list will be returned. A null entry
    /// represents an empty line and an empty String represents an empty comment
    /// line.
    pub fn footer(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let temp_clone =
            crate::configuration::file::FileConfigurationOptions::from_raw(&self.0, unsafe {
                jni::objects::JObject::from_raw(self.1.clone())
            })?;
        let real: crate::configuration::file::FileConfigurationOptions = temp_clone.into();
        real.footer()
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::configuration::file::FileConfigurationOptions<'mc>>
    for YamlConfigurationOptions<'mc>
{
    fn into(self) -> crate::configuration::file::FileConfigurationOptions<'mc> {
        crate::configuration::file::FileConfigurationOptions::from_raw(&self.jni_ref(), self.1).expect("Error converting YamlConfigurationOptions into crate::configuration::file::FileConfigurationOptions")
    }
}

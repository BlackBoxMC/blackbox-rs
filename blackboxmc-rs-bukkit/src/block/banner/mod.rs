#![allow(deprecated)]
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum PatternTypeEnum {
    Base,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    StripeBottom,
    StripeTop,
    StripeLeft,
    StripeRight,
    StripeCenter,
    StripeMiddle,
    StripeDownright,
    StripeDownleft,
    StripeSmall,
    Cross,
    StraightCross,
    TriangleBottom,
    TriangleTop,
    TrianglesBottom,
    TrianglesTop,
    DiagonalLeft,
    DiagonalRight,
    DiagonalLeftMirror,
    DiagonalRightMirror,
    CircleMiddle,
    RhombusMiddle,
    HalfVertical,
    HalfHorizontal,
    HalfVerticalMirror,
    HalfHorizontalMirror,
    Border,
    CurlyBorder,
    Creeper,
    Gradient,
    GradientUp,
    Bricks,
    Skull,
    Flower,
    Mojang,
    Globe,
    Piglin,
}
impl std::fmt::Display for PatternTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PatternTypeEnum::Base => f.write_str("BASE"),
            PatternTypeEnum::SquareBottomLeft => f.write_str("SQUARE_BOTTOM_LEFT"),
            PatternTypeEnum::SquareBottomRight => f.write_str("SQUARE_BOTTOM_RIGHT"),
            PatternTypeEnum::SquareTopLeft => f.write_str("SQUARE_TOP_LEFT"),
            PatternTypeEnum::SquareTopRight => f.write_str("SQUARE_TOP_RIGHT"),
            PatternTypeEnum::StripeBottom => f.write_str("STRIPE_BOTTOM"),
            PatternTypeEnum::StripeTop => f.write_str("STRIPE_TOP"),
            PatternTypeEnum::StripeLeft => f.write_str("STRIPE_LEFT"),
            PatternTypeEnum::StripeRight => f.write_str("STRIPE_RIGHT"),
            PatternTypeEnum::StripeCenter => f.write_str("STRIPE_CENTER"),
            PatternTypeEnum::StripeMiddle => f.write_str("STRIPE_MIDDLE"),
            PatternTypeEnum::StripeDownright => f.write_str("STRIPE_DOWNRIGHT"),
            PatternTypeEnum::StripeDownleft => f.write_str("STRIPE_DOWNLEFT"),
            PatternTypeEnum::StripeSmall => f.write_str("STRIPE_SMALL"),
            PatternTypeEnum::Cross => f.write_str("CROSS"),
            PatternTypeEnum::StraightCross => f.write_str("STRAIGHT_CROSS"),
            PatternTypeEnum::TriangleBottom => f.write_str("TRIANGLE_BOTTOM"),
            PatternTypeEnum::TriangleTop => f.write_str("TRIANGLE_TOP"),
            PatternTypeEnum::TrianglesBottom => f.write_str("TRIANGLES_BOTTOM"),
            PatternTypeEnum::TrianglesTop => f.write_str("TRIANGLES_TOP"),
            PatternTypeEnum::DiagonalLeft => f.write_str("DIAGONAL_LEFT"),
            PatternTypeEnum::DiagonalRight => f.write_str("DIAGONAL_RIGHT"),
            PatternTypeEnum::DiagonalLeftMirror => f.write_str("DIAGONAL_LEFT_MIRROR"),
            PatternTypeEnum::DiagonalRightMirror => f.write_str("DIAGONAL_RIGHT_MIRROR"),
            PatternTypeEnum::CircleMiddle => f.write_str("CIRCLE_MIDDLE"),
            PatternTypeEnum::RhombusMiddle => f.write_str("RHOMBUS_MIDDLE"),
            PatternTypeEnum::HalfVertical => f.write_str("HALF_VERTICAL"),
            PatternTypeEnum::HalfHorizontal => f.write_str("HALF_HORIZONTAL"),
            PatternTypeEnum::HalfVerticalMirror => f.write_str("HALF_VERTICAL_MIRROR"),
            PatternTypeEnum::HalfHorizontalMirror => f.write_str("HALF_HORIZONTAL_MIRROR"),
            PatternTypeEnum::Border => f.write_str("BORDER"),
            PatternTypeEnum::CurlyBorder => f.write_str("CURLY_BORDER"),
            PatternTypeEnum::Creeper => f.write_str("CREEPER"),
            PatternTypeEnum::Gradient => f.write_str("GRADIENT"),
            PatternTypeEnum::GradientUp => f.write_str("GRADIENT_UP"),
            PatternTypeEnum::Bricks => f.write_str("BRICKS"),
            PatternTypeEnum::Skull => f.write_str("SKULL"),
            PatternTypeEnum::Flower => f.write_str("FLOWER"),
            PatternTypeEnum::Mojang => f.write_str("MOJANG"),
            PatternTypeEnum::Globe => f.write_str("GLOBE"),
            PatternTypeEnum::Piglin => f.write_str("PIGLIN"),
        }
    }
}
pub struct PatternType<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
    pub PatternTypeEnum,
);
impl<'mc> std::ops::Deref for PatternType<'mc> {
    type Target = PatternTypeEnum;
    fn deref(&self) -> &Self::Target {
        return &self.2;
    }
}
impl<'mc> JNIRaw<'mc> for PatternType<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> PatternType<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
        e: PatternTypeEnum,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate PatternType from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "PatternType")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a PatternType object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj, e))
        }
    }
    pub const BASE: PatternTypeEnum = PatternTypeEnum::Base;
    pub const SQUARE_BOTTOM_LEFT: PatternTypeEnum = PatternTypeEnum::SquareBottomLeft;
    pub const SQUARE_BOTTOM_RIGHT: PatternTypeEnum = PatternTypeEnum::SquareBottomRight;
    pub const SQUARE_TOP_LEFT: PatternTypeEnum = PatternTypeEnum::SquareTopLeft;
    pub const SQUARE_TOP_RIGHT: PatternTypeEnum = PatternTypeEnum::SquareTopRight;
    pub const STRIPE_BOTTOM: PatternTypeEnum = PatternTypeEnum::StripeBottom;
    pub const STRIPE_TOP: PatternTypeEnum = PatternTypeEnum::StripeTop;
    pub const STRIPE_LEFT: PatternTypeEnum = PatternTypeEnum::StripeLeft;
    pub const STRIPE_RIGHT: PatternTypeEnum = PatternTypeEnum::StripeRight;
    pub const STRIPE_CENTER: PatternTypeEnum = PatternTypeEnum::StripeCenter;
    pub const STRIPE_MIDDLE: PatternTypeEnum = PatternTypeEnum::StripeMiddle;
    pub const STRIPE_DOWNRIGHT: PatternTypeEnum = PatternTypeEnum::StripeDownright;
    pub const STRIPE_DOWNLEFT: PatternTypeEnum = PatternTypeEnum::StripeDownleft;
    pub const STRIPE_SMALL: PatternTypeEnum = PatternTypeEnum::StripeSmall;
    pub const CROSS: PatternTypeEnum = PatternTypeEnum::Cross;
    pub const STRAIGHT_CROSS: PatternTypeEnum = PatternTypeEnum::StraightCross;
    pub const TRIANGLE_BOTTOM: PatternTypeEnum = PatternTypeEnum::TriangleBottom;
    pub const TRIANGLE_TOP: PatternTypeEnum = PatternTypeEnum::TriangleTop;
    pub const TRIANGLES_BOTTOM: PatternTypeEnum = PatternTypeEnum::TrianglesBottom;
    pub const TRIANGLES_TOP: PatternTypeEnum = PatternTypeEnum::TrianglesTop;
    pub const DIAGONAL_LEFT: PatternTypeEnum = PatternTypeEnum::DiagonalLeft;
    pub const DIAGONAL_RIGHT: PatternTypeEnum = PatternTypeEnum::DiagonalRight;
    pub const DIAGONAL_LEFT_MIRROR: PatternTypeEnum = PatternTypeEnum::DiagonalLeftMirror;
    pub const DIAGONAL_RIGHT_MIRROR: PatternTypeEnum = PatternTypeEnum::DiagonalRightMirror;
    pub const CIRCLE_MIDDLE: PatternTypeEnum = PatternTypeEnum::CircleMiddle;
    pub const RHOMBUS_MIDDLE: PatternTypeEnum = PatternTypeEnum::RhombusMiddle;
    pub const HALF_VERTICAL: PatternTypeEnum = PatternTypeEnum::HalfVertical;
    pub const HALF_HORIZONTAL: PatternTypeEnum = PatternTypeEnum::HalfHorizontal;
    pub const HALF_VERTICAL_MIRROR: PatternTypeEnum = PatternTypeEnum::HalfVerticalMirror;
    pub const HALF_HORIZONTAL_MIRROR: PatternTypeEnum = PatternTypeEnum::HalfHorizontalMirror;
    pub const BORDER: PatternTypeEnum = PatternTypeEnum::Border;
    pub const CURLY_BORDER: PatternTypeEnum = PatternTypeEnum::CurlyBorder;
    pub const CREEPER: PatternTypeEnum = PatternTypeEnum::Creeper;
    pub const GRADIENT: PatternTypeEnum = PatternTypeEnum::Gradient;
    pub const GRADIENT_UP: PatternTypeEnum = PatternTypeEnum::GradientUp;
    pub const BRICKS: PatternTypeEnum = PatternTypeEnum::Bricks;
    pub const SKULL: PatternTypeEnum = PatternTypeEnum::Skull;
    pub const FLOWER: PatternTypeEnum = PatternTypeEnum::Flower;
    pub const MOJANG: PatternTypeEnum = PatternTypeEnum::Mojang;
    pub const GLOBE: PatternTypeEnum = PatternTypeEnum::Globe;
    pub const PIGLIN: PatternTypeEnum = PatternTypeEnum::Piglin;
    pub fn from_string(str: String) -> std::option::Option<PatternTypeEnum> {
        match str.as_str() {
            "BASE" => Some(PatternTypeEnum::Base),
            "SQUARE_BOTTOM_LEFT" => Some(PatternTypeEnum::SquareBottomLeft),
            "SQUARE_BOTTOM_RIGHT" => Some(PatternTypeEnum::SquareBottomRight),
            "SQUARE_TOP_LEFT" => Some(PatternTypeEnum::SquareTopLeft),
            "SQUARE_TOP_RIGHT" => Some(PatternTypeEnum::SquareTopRight),
            "STRIPE_BOTTOM" => Some(PatternTypeEnum::StripeBottom),
            "STRIPE_TOP" => Some(PatternTypeEnum::StripeTop),
            "STRIPE_LEFT" => Some(PatternTypeEnum::StripeLeft),
            "STRIPE_RIGHT" => Some(PatternTypeEnum::StripeRight),
            "STRIPE_CENTER" => Some(PatternTypeEnum::StripeCenter),
            "STRIPE_MIDDLE" => Some(PatternTypeEnum::StripeMiddle),
            "STRIPE_DOWNRIGHT" => Some(PatternTypeEnum::StripeDownright),
            "STRIPE_DOWNLEFT" => Some(PatternTypeEnum::StripeDownleft),
            "STRIPE_SMALL" => Some(PatternTypeEnum::StripeSmall),
            "CROSS" => Some(PatternTypeEnum::Cross),
            "STRAIGHT_CROSS" => Some(PatternTypeEnum::StraightCross),
            "TRIANGLE_BOTTOM" => Some(PatternTypeEnum::TriangleBottom),
            "TRIANGLE_TOP" => Some(PatternTypeEnum::TriangleTop),
            "TRIANGLES_BOTTOM" => Some(PatternTypeEnum::TrianglesBottom),
            "TRIANGLES_TOP" => Some(PatternTypeEnum::TrianglesTop),
            "DIAGONAL_LEFT" => Some(PatternTypeEnum::DiagonalLeft),
            "DIAGONAL_RIGHT" => Some(PatternTypeEnum::DiagonalRight),
            "DIAGONAL_LEFT_MIRROR" => Some(PatternTypeEnum::DiagonalLeftMirror),
            "DIAGONAL_RIGHT_MIRROR" => Some(PatternTypeEnum::DiagonalRightMirror),
            "CIRCLE_MIDDLE" => Some(PatternTypeEnum::CircleMiddle),
            "RHOMBUS_MIDDLE" => Some(PatternTypeEnum::RhombusMiddle),
            "HALF_VERTICAL" => Some(PatternTypeEnum::HalfVertical),
            "HALF_HORIZONTAL" => Some(PatternTypeEnum::HalfHorizontal),
            "HALF_VERTICAL_MIRROR" => Some(PatternTypeEnum::HalfVerticalMirror),
            "HALF_HORIZONTAL_MIRROR" => Some(PatternTypeEnum::HalfHorizontalMirror),
            "BORDER" => Some(PatternTypeEnum::Border),
            "CURLY_BORDER" => Some(PatternTypeEnum::CurlyBorder),
            "CREEPER" => Some(PatternTypeEnum::Creeper),
            "GRADIENT" => Some(PatternTypeEnum::Gradient),
            "GRADIENT_UP" => Some(PatternTypeEnum::GradientUp),
            "BRICKS" => Some(PatternTypeEnum::Bricks),
            "SKULL" => Some(PatternTypeEnum::Skull),
            "FLOWER" => Some(PatternTypeEnum::Flower),
            "MOJANG" => Some(PatternTypeEnum::Mojang),
            "GLOBE" => Some(PatternTypeEnum::Globe),
            "PIGLIN" => Some(PatternTypeEnum::Piglin),
            _ => None,
        }
    }
    pub fn identifier(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getIdentifier",
            "()Ljava/lang/String;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn get_by_identifier(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/block/banner/PatternType")?;
        let res = jni.call_static_method(
            cls,
            "getByIdentifier",
            "(Ljava/lang/String;)Lorg/bukkit/block/banner/PatternType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::banner::PatternType::from_raw(
            &jni,
            raw_obj,
            crate::block::banner::PatternType::from_string(variant_str).unwrap(),
        )
    }
    pub fn value_of(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<&'mc String>,
    ) -> Result<crate::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JObject::from(jni.new_string(arg0.into()).unwrap());
        let cls = &jni.find_class("org/bukkit/block/banner/PatternType")?;
        let res = jni.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/block/banner/PatternType;",
            &[jni::objects::JValueGen::from(&val_0)],
        )?;
        let mut obj = res.l()?;
        let raw_obj = obj;
        let variant = jni.call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = jni
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::banner::PatternType::from_raw(
            &jni,
            raw_obj,
            crate::block::banner::PatternType::from_string(variant_str).unwrap(),
        )
    }
}
pub struct Pattern<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);
impl<'mc> blackboxmc_general::JNIRaw<'mc> for Pattern<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }

    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> Pattern<'mc> {
    pub fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Pattern from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "Pattern")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Pattern object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
    pub fn new_with_map(
        jni: blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: std::option::Option<impl Into<&'mc crate::DyeColor<'mc>>>,
        arg1: std::option::Option<impl Into<&'mc crate::block::banner::PatternType<'mc>>>,
    ) -> Result<crate::block::banner::Pattern<'mc>, Box<dyn std::error::Error>> {
        let val_0 =
            unsafe { jni::objects::JObject::from_raw(arg0.unwrap().into().jni_object().clone()) };
        let val_1 =
            unsafe { jni::objects::JObject::from_raw(arg1.unwrap().into().jni_object().clone()) };
        let cls = &jni.find_class("org/bukkit/block/banner/Pattern")?;
        let res = jni.new_object(
            cls,
            "(Lorg/bukkit/DyeColor;Lorg/bukkit/block/banner/PatternType;)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        )?;
        crate::block::banner::Pattern::from_raw(&jni, res)
    }
    pub fn serialize(
        &mut self,
    ) -> Result<blackboxmc_javautil::JavaMap<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "serialize", "()Ljava/util/Map;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        blackboxmc_javautil::JavaMap::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    pub fn color(&mut self) -> Result<crate::DyeColor<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getColor",
            "()Lorg/bukkit/DyeColor;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::DyeColor::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::DyeColor::from_string(variant_str).unwrap(),
        )
    }
    pub fn pattern(
        &mut self,
    ) -> Result<crate::block::banner::PatternType<'mc>, Box<dyn std::error::Error>> {
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getPattern",
            "()Lorg/bukkit/block/banner/PatternType;",
            &[],
        );
        let res = self.jni_ref().translate_error(res)?;
        let raw_obj = unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) };
        let variant = self
            .0
            .call_method(&raw_obj, "toString", "()Ljava/lang/String;", &[])?;
        let variant_str = self
            .0
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        crate::block::banner::PatternType::from_raw(
            &self.jni_ref(),
            raw_obj,
            crate::block::banner::PatternType::from_string(variant_str).unwrap(),
        )
    }
    pub fn equals(
        &mut self,
        arg0: jni::objects::JObject<'mc>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let val_0 = arg0;
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "equals",
            "(Ljava/lang/Object;)Z",
            &[jni::objects::JValueGen::from(&val_0)],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.z().unwrap())
    }
    pub fn hash_code(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "hashCode", "()I", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i().unwrap())
    }
    pub fn wait(
        &mut self,
        arg0: std::option::Option<i64>,
        arg1: std::option::Option<i32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let val_0 = jni::objects::JValueGen::Long(arg0.unwrap().into());
        let val_1 = jni::objects::JValueGen::Int(arg1.unwrap().into());
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "wait",
            "(JI)V",
            &[
                jni::objects::JValueGen::from(&val_0),
                jni::objects::JValueGen::from(&val_1),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn to_string(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "toString", "()Ljava/lang/String;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(self
            .jni_ref()
            .get_string(unsafe { &jni::objects::JString::from_raw(res.as_jni().l) })?
            .to_string_lossy()
            .to_string())
    }
    pub fn class(&mut self) -> Result<jni::objects::JClass<'mc>, Box<dyn std::error::Error>> {
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getClass", "()Ljava/lang/Class;", &[]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(unsafe { jni::objects::JClass::from_raw(res.as_jni().l) })
    }
    pub fn notify(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notify", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    pub fn notify_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "notifyAll", "()V", &[]);
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
}
impl<'mc> Into<crate::configuration::serialization::ConfigurationSerializable<'mc>>
    for Pattern<'mc>
{
    fn into(self) -> crate::configuration::serialization::ConfigurationSerializable<'mc> {
        crate::configuration::serialization::ConfigurationSerializable::from_raw(
            &self.jni_ref(),
            self.1,
        )
        .unwrap()
    }
}
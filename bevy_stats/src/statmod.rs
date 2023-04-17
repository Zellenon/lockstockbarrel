use std::marker::PhantomData;

use crate::{RPGResource, RPGStat};

pub enum ModType {
    Offset,
    Multiplier,
}

pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
}

pub enum MultiplierStyle {
    Additive,
    Multiplicative,
}

pub enum ResourceModUpdateStyle {
    ScaleBoth,
    ScaleOnBuff,
    ScaleOnDebuff,
    NoScale,
}

pub struct ValueChange<T>
where
    T: RPGStat,
{
    category: ModType,
    value: f32,
    _phantom: PhantomData<T>,
}

pub struct ResourceChange<T>
where
    T: RPGResource,
{
    category: ModType,
    value: f32,
    _phantom: PhantomData<T>,
}

pub struct StatModifier<T>
where
    T: RPGStat,
{
    mod_type: ModType,
    value: f32,
    _phantom: PhantomData<T>,
}

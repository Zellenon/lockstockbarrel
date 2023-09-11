use bevy::reflect::{FieldIter, Reflect, Struct};

// trait Delta<'a> {
//     fn delta(self) -> Vec<(String, Box<&'a dyn Reflect>)>;
// }

#[derive(Clone, Copy, PartialEq)]
struct Delta<T: Reflect + Default + Struct + Clone>(T);

impl<T: Reflect + Default + Struct + Clone> Delta<T> {
    fn apply(&self, other: &T) -> T {
        let fields = fields(&self.0.clone());
        let default = T::default();
        let mut diffs = Vec::new();
        for field in fields.iter() {
            if let Some(false) = self
                .0
                .field(field)
                .unwrap()
                .reflect_partial_eq(default.field(field).unwrap())
            {
                let val = (&self.0.clone()).field(field).unwrap().clone();
                diffs.push((field.clone(), Box::new(val)));
            }
        }
    }
}

fn fields<'a>(a: &impl Struct) -> Vec<String> {
    let count = a.field_len();
    // (0..count).map(|w| a.name_at(w).unwrap()).collect()
    let mut vec = Vec::new();
    for i in 0..count {
        vec.push({
            let a = a.name_at(i).unwrap();
            let b: String = a.into();
            b.clone()
        });
    }
    vec
}

// impl<'a, T> Delta<'a> for T
// where
//     T: Reflect + Default + Struct + Clone,
// {
//     fn delta(self) -> Vec<(String, Box<&'a dyn Reflect>)> {
//         let fields = fields(&self.clone());
//         let default = T::default();
//         let mut diffs = Vec::new();
//         for field in fields.iter() {
//             if let Some(false) = self
//                 .field(field)
//                 .unwrap()
//                 .reflect_partial_eq(default.field(field).unwrap())
//             {
//                 let val = (&self.clone()).field(field).unwrap().clone();
//                 diffs.push((field.clone(), Box::new(val)));
//             }
//         }
//         diffs
//     }
// }

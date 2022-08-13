pub struct SampleStruct{
    pub sample_field: i32
}
impl SampleStruct {
    pub fn new_sample() -> SampleStruct{
        SampleStruct { sample_field: -1 }
    }
}
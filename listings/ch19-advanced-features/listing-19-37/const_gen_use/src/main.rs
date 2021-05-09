use std::ops::Add;
use std::marker::PhantomData;

struct GPUId<const ID: u32>;

struct CPUId<const ID: u32>;

trait DeviceIdTrait {}

impl<const ID:u32>DeviceIdTrait for GPUId<ID> {}

impl<const ID:u32>DeviceIdTrait for CPUId<ID> {}

struct NumberWrapper<T : DeviceIdTrait> {
    num : u64,
    _phantom : PhantomData<T>
}

impl<T:DeviceIdTrait> NumberWrapper<T> {
    fn new(num : u64) -> Self {
        NumberWrapper {
            num,
            _phantom: PhantomData
        }
    }
}

impl<T:DeviceIdTrait>Add for NumberWrapper<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        NumberWrapper::new(self.num + rhs.num)
    }
}


fn main() {

    let x : NumberWrapper<CPUId<0>> = NumberWrapper::new(5);
    let y : NumberWrapper<CPUId<0>> = NumberWrapper::new(6);
    let z = x+y;
    assert_eq!(11,z.num);
}
 //генерация отсортированого кортежа u8 из 32 чисел
 use rand::{thread_rng, Rng};
 
pub fn generation () -> Vec<u8> {
    let mut rng = thread_rng();
    let mut arr1: [u8; 32] = rng.gen();
    arr1.sort();
    arr1.to_vec()
}
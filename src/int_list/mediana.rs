    // вывод медианы списка числел
pub fn mediana (arr1: &Vec<u8>) -> u8 { 
    let index_mediana: usize = &arr1.len() / 2;
    let mediana: &u8 = &arr1[index_mediana];
    *mediana
    
}
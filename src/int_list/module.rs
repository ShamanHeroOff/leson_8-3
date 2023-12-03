use std::collections::HashMap;
use std::cmp::Ordering;

  //вывод модуля списка числе
pub fn module (arr1: &Vec<u8>) -> Vec<u8> {
    let mut module: HashMap<u8, u8> = HashMap::new();
    let mut res: Vec<u8> = Vec::new(); 

    for k in arr1 {
        let count = module.entry(*k).or_default();
        *count += 1;
    } 

    let mut count: u8 = 1; 

    for (k, v) in &module {
        match v.cmp(&count){
            Ordering::Less => continue,
            Ordering::Equal => res.push(*k),
            Ordering::Greater => {
                res.clear();
                res.push(*k);
                count = *k
            }
        }
    }
    res
}

// если > то вектор очищается и добавляется новый элемент

// если < переходим на следующий цикл 

// если == то пушим в вектор 

// если математика окажется шуткой, то вернем просто пустой Vec!
// Сгенерировать список 'n' целых чисел и сохранить +
// Отсортировать список по возрастанию +
// Функция которая возвращает среднее значение списка - медиану (использовать вектор); +
// вернуть моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз; HashMap будет полезна в данном случае).+
// расскидать по функциям и после раскидать по модуям

mod int_list;


fn main() {
    // генерация вектора для дальнейшей работы над ним
    let vec = int_list::generation::generation();
    println!("The is Vec! =>{:#?}", vec);

    // вывод медианы списка числел
    println!("The is mediana =>{}", int_list::mediana::mediana(&vec));
    
    //вывод модуля списка числе
    println!("The is module list =>{:?}", int_list::module::module(&vec));
}

// улучшить выражение модумя списка, что бы при насличии 
// дву повторяющихся числах, выводились они оба
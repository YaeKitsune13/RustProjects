use std::io::stdin; // импортируем беблиотеку для работы текстом в консоли

fn main() {
    // Задаём точку входа
    let mut notes = Vec::new(); // создаём изменяемый вектор для заметок

    loop {
        // создаём что то на подобий while true
        print_menu(); // выводим menu

        let command = read_input(); // получаем пользовательский ввод в консоль

        match command.trim() {
            // удаляем лишние символы по типу /n
            "show" => show_notes(&notes), // вызываем метод показа заметок
            "add" => add_note(&mut notes), // вызываем метод добавления заметки
            _ => break,                   // иначе выходим из цикла
        }
    }
}

fn print_menu() {
    println!();
    println!();
    println!("**** PROGRAM MENU ****");
    println!("Enter command:");
    println!("'show' - show all notes");
    println!("'add' - add new note");
    println!("other - exit");
}

fn read_input() -> String {
    let mut buffer = String::new(); // создаём строку в которую будем записывать пользовательский ввод

    stdin().read_line(&mut buffer).unwrap(); // с помощьб stdin считываем строку и записываем её в переменную buffer удаляя пробелы по краям

    buffer // return buffer
}

fn show_notes(notes: &[String]) {
    println!();

    for note in notes {
        println!("{}", note)
    }
}

fn add_note(notes: &mut Vec<String>) {
    println!();
    println!("Enter note:");

    let input = read_input();

    let note = input.trim().to_string(); // удаляем симолы /n и превращяем в строку

    notes.push(note);
}

/*
Faktanya, Printings macros dilengkapi dengan std::fmt, yang menyediakan berbagai macam macro untuk mencetak teks dengan format tertentu. Beberapa macro yang umum digunakan adalah


- format!: menghasilkan String yang diformat sesuai dengan argumen yang diberikan. Ini tidak mencetak apa pun ke konsol, tetapi mengembalikan String yang dapat digunakan untuk tujuan lain.

- print!: Sama seperti format! tetapi dicetak ke console (io::stdout).

- println!: Sama seperti format! tetapi menambahkan baris baru di akhir.

- eprint!: Sama seperti print! tetapi teks dicetak ke error output (io::stderr).

- eprintln!: Sama seperti eprint! tetapi menambahkan baris baru di akhir.
*/

fn main() {
    // Pada umumnya, simbol {} digunakan sebagai placeholder untuk argumen yang akan diformat. Anda dapat menggunakan beberapa simbol {} dalam satu macro untuk mencetak beberapa argumen sekaligus.

    println!("Hello, namaku {}, umurku {} tahun.", "Alice", 30); // Output: Hello, namaku Alice, umurku 30 tahun.

    // Hanya pembatas
    println!("---");

    // Perbedaan setiap Printing macro
    // format! menghasilkan String yang diformat sesuai dengan argumen yang diberikan. Ini tidak mencetak apa pun ke konsol, tetapi mengembalikan String yang dapat digunakan untuk tujuan lain.
    let formated_string = format!("Hello, namaku {}, umurku {} tahun.", "Bob", 25); // Menghasilkan String yang diformat
    println!("{}", formated_string); // Output: Hello, namaku Bob, umurku 25 tahun.

    // print! Sama seperti format! tetapi dicetak ke console (io::stdout).
    print!("Hello, namaku {}, umurku {} tahun.", "Charlie", 28); // Output: Hello, namaku Charlie, umurku 28 tahun.
    print!(" - Tanpa baris baru\n"); // Output:  - Tanpa baris baru (Note: \n digunakan untuk membuat baris baru)

    // println! Sama seperti format! tetapi menambahkan baris baru di akhir.
    println!("Hello, namaku {}, umurku {} tahun.", "Dave", 22); // Output: Hello, namaku Dave, umurku 22 tahun.
    println!(" - Dengan baris baru"); // Output:  - Dengan baris baru

    // eprint! Sama seperti print! tetapi teks dicetak ke error output (io::stderr).
    eprint!("Error type: {}, Message: {}", "Syntax error", "Invalid syntax in line 35"); // Output: Error type: Syntax error, Message: Invalid syntax in line 35 (Dicetak ke error output)
    eprint!(" - Tanpa baris baru\n"); // Output:  - Tanpa baris baru (Dicetak ke error output)

    // eprintln! Sama seperti eprint! tetapi menambahkan baris baru di akhir.
    eprintln!("Error type: {}, Message: {}", "Runtime error", "Division by zero"); // Output: Error type: Runtime error, Message: Division by zero (Dicetak ke error output)
    eprintln!(" - Dengan baris baru"); // Output:  - Dengan baris baru (Dicetak ke error output)
}

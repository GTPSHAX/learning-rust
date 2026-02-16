/*
Faktanya, Printings macros dilengkapi dengan std::fmt, yang menyediakan berbagai macam macro untuk mencetak teks dengan format tertentu. Beberapa macro yang umum digunakan adalah


- format!: menghasilkan String yang diformat sesuai dengan argumen yang diberikan. Ini tidak mencetak apa pun ke konsol, tetapi mengembalikan String yang dapat digunakan untuk tujuan lain.

- print!: Sama seperti format! tetapi dicetak ke console (io::stdout).

- println!: Sama seperti format! tetapi menambahkan baris baru di akhir.

- eprint!: Sama seperti print! tetapi teks dicetak ke error output (io::stderr).

- eprintln!: Sama seperti eprint! tetapi menambahkan baris baru di akhir.
*/

/*
std::fmt punya banyak trait yang ngatur cara teks ditampilkan. Dua yang penting:


- fmt::Debug : pakai penanda {:?}. Buat format teks untuk debugging.

- fmt::Display : pakai {}. Buat format yang lebih rapi dan ramah user.
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
    eprint!("Error type: {:?}, Message: {:?}", "Syntax error", "Invalid syntax in line 35"); // Output: Error type: Syntax error, Message: Invalid syntax in line 35 (Dicetak ke error output)
    eprint!(" - Tanpa baris baru\n"); // Output:  - Tanpa baris baru (Dicetak ke error output)

    // eprintln! Sama seperti eprint! tetapi menambahkan baris baru di akhir.
    eprintln!("Error type: {:?}, Message: {:?}", "Runtime error", "Division by zero"); // Output: Error type: Runtime error, Message: Division by zero (Dicetak ke error output)
    eprintln!(" - Dengan baris baru"); // Output:  - Dengan baris baru (Dicetak ke error output)


    // Kita dapat memangil kembali argumen yang sama beberapa kali dalam satu macro dengan menggunakan indeks posisi. Misalnya, kita bisa menggunakan {0} untuk merujuk ke argumen pertama, {1} untuk argumen kedua, dan seterusnya.
    println!("Hello, {0}! Your name is {0} and you are {1} years old.", "Eve", 27); // Output: Hello, Eve! Your name is Eve and you are 27 years old.

    // Dapat juga menggunakan nama argumen untuk merujuk ke nilai yang diberikan. Misalnya, kita bisa menggunakan {name} untuk merujuk ke argumen dengan nama "name".
    println!("Hello, {name}! Your name is {name} and you are {age} years old.", name = "Frank", age = 35); // Output: Hello, Frank! Your name is Frank and you are 35 years old.

    // Dapat juga membuat format yang lebih kompleks dengan menggunakan format spesifikasi. Misalnya, kita bisa menggunakan {:>10} untuk meratakan teks ke kanan dengan lebar 10 karakter, atau {:0>5} untuk mengisi angka dengan nol di depan hingga mencapai lebar 5 karakter.
    println!("Name: {:>10}, Age: {:0>5}", "Grace", 29); // Output: Name:      Grace, Age: 00029
    println!("{number:0>5}", number = 42); // Output: 00042

    // Mencetak angka dengan format tertentu, seperti bilangan bulat, bilangan desimal, atau bilangan dalam format ilmiah.
    println!("Integer: {}, Float: {:.2}, Scientific: {:e}", 123, 3.14159, 0.00123); // Output: Integer: 123, Float: 3.14, Scientific: 1.23e-3
    println!("Base 10:          {}", 69420); // Output: Base 10:          69420
    println!("Base 2 (binary):  {:b}", 69420); // Output: Base 2 (binary):  10000111100101100
    println!("Base 8 (octal):   {:o}", 69420); // Output: Base 8 (octal):   207454
    println!("Base 16 (hex):    {:x}", 69420); // Output: Base 16 (hex):    10f2

    // Rust version 1.58 kita dapat langsung menggunakan variabel dalam format string tanpa harus menyebutkannya sebagai argumen tambahan. Misalnya, kita bisa menggunakan {name} untuk merujuk ke variabel name yang sudah didefinisikan sebelumnya.
    let name = "Heidi";
    let age = 31;
    println!("Hello, {name}! Your name is {name} and you are {age} years old."); // Output: Hello, Heidi! Your name is Heidi and you are 31 years old.
}

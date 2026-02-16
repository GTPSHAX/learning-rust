// Sebuah string format wajib menggunakan semua argumen yang diberikan. Jika ada argumen yang tidak digunakan dalam string format, maka Rust akan menghasilkan error saat kompilasi. Hal ini dilakukan untuk memastikan bahwa semua argumen yang diberikan benar-benar digunakan dalam string format, sehingga mencegah kesalahan atau ketidaksesuaian dalam output yang dihasilkan.

fn main() {
    // Pada umumnya, simbol {} digunakan sebagai placeholder untuk argumen yang akan diformat. Anda dapat menggunakan beberapa simbol {} dalam satu macro untuk mencetak beberapa argumen sekaligus. Setiap simbol {} akan digantikan dengan argumen yang sesuai dalam urutan yang diberikan. Misalnya, jika Anda memiliki dua simbol {} dalam satu macro, maka argumen pertama akan menggantikan simbol {} pertama, dan argumen kedua akan menggantikan simbol {} kedua.

    println!("{} {}", 1, 2); // Output: 1 2

    // Tetapi kita juga bisa menggunakan indeks posisi untuk merujuk ke argumen tertentu. Misalnya, kita bisa menggunakan {0} untuk merujuk ke argumen pertama, {1} untuk argumen kedua, dan seterusnya. Ini memungkinkan kita untuk menggunakan argumen yang sama beberapa kali dalam satu macro tanpa harus mengulanginya.

    println!("{0} {1} {0}", 1, 2); // Output: 1 2 1

    // Tetapi hal ini dapat menjadi hal yang sedikit rumit jika kedua jenis penentu opsi digunakan dalam satu macro. Misalnya, jika kita menggunakan {0} untuk merujuk ke argumen pertama, tetapi juga menggunakan {} tanpa indeks posisi, maka Rust akan menganggap bahwa {} merujuk ke argumen pertama, bukan argumen kedua. Oleh karena itu, disarankan untuk menggunakan salah satu jenis penentu opsi secara konsisten dalam satu macro untuk menghindari kebingungan.

    println!("{1} {} {0} {}", 1, 2); // Output: 2 1 1 2 (Note: {} merujuk ke argumen pertama, yaitu 1, dan {0} merujuk ke argumen pertama juga, yaitu 1. Argumen kedua, yaitu 2, digunakan untuk {1} dan {} yang kedua.)

    // Macro format! adalah ektensi syntax yang memungkinkan kita untuk menggunakan nama argumen untuk merujuk ke nilai yang diberikan. Misalnya, kita bisa menggunakan {name} untuk merujuk ke argumen dengan nama "name". Ini membuat kode lebih mudah dibaca dan dipahami, terutama jika kita memiliki banyak argumen yang digunakan dalam satu macro.

    format!("{argument}", argument = "test"); // Output: "test" (Note: {argument} merujuk ke argumen dengan nama "argument", yang memiliki nilai "test".)
    format!("{name} {}", 1, name = 2); // Output: "2 1" (Note: {name} merujuk ke argumen dengan nama "name", yang memiliki nilai 2, dan {} merujuk ke argumen pertama, yaitu 1.)
    format!("{a} {c} {b}", a="a", b='b', c=3); // Output: "a 3 b" (Note: {a} merujuk ke argumen dengan nama "a", yang memiliki nilai "a", {c} merujuk ke argumen dengan nama "c", yang memiliki nilai 3, dan {b} merujuk ke argumen dengan nama "b", yang memiliki nilai 'b'.)

    // TODO: Capek ah, lanjut nanti, mau skroll fesnuk
}

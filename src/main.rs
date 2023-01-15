

struct  Kutuphane{
    kitaplar: Vec<Kitap>,
}

struct  Kitap{
    baslik: String,
    yil: u16,
}

impl Kitap{
    fn new(baslik:&str,yil: u16) -> Kitap {
        Kitap{
            baslik:String::from(baslik),
            yil,
        }
    }
}

impl std::fmt::Display for Kitap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.baslik, self.yil)
    }
}

impl Kutuphane{
    fn new() -> Kutuphane {
        Kutuphane{
            kitaplar: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.kitaplar.len()
    }

    fn is_empty(&self) -> bool {
        self.kitaplar.is_empty()
    }

    fn kitap_ekle(&mut self,Kitap:Kitap) {
        self.kitaplar.push(Kitap);
    }

    fn kitaplari_bastir(&self){
        for kitap in &self.kitaplar {
            println!("{}",kitap);
        }

    }

    fn en_eski_kitap(&self)-> Option<&Kitap>{
        let mut en_eski_kitap =self.kitaplar.get(0);
        for kitap in &self.kitaplar {
            if kitap.yil < en_eski_kitap.unwrap().yil {
                en_eski_kitap = Some(&kitap);
            }
        }
        en_eski_kitap
    }

}



fn main() {
    let mut 
    kutuphane= Kutuphane::new();

    println!("Kütüphane Boş mu? {}",match kutuphane.is_empty() {
        true => "Evet",
        false => "Hayır",
    });
        
    kutuphane.kitap_ekle(Kitap::new("Rust",2010));
    kutuphane.kitap_ekle(Kitap::new("C++",1985));
    kutuphane.kitap_ekle(Kitap::new("Lord of the Rings",1954));
    kutuphane.kitap_ekle(Kitap::new("Harry Potter",1997));

    kutuphane.kitaplari_bastir();

    match kutuphane.en_eski_kitap() {
        Some(kitap) => println!("En Eski Kitap: {}",kitap),
        None => println!("Kitap Yok"),
        
    }

    println!("Kütüphanedeki Kitap Sayısı: {}",kutuphane.len());
}

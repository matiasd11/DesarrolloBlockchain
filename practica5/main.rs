struct ConcesionarioAuto{
    nombre: String,
    direccion: String,
    capacidad_maxima: u8,
    autos: Vec<Auto>,
}
#[derive(Clone, PartialEq, Debug)]
struct Auto{
    marca: String,
    modelo: String,
    anio: u16,
    precio: f64,
    color: Color,
}
#[derive(Clone, PartialEq, Debug)]
enum Color{
    Rojo,
    Azul,
    Verde,
    Amarillo,
    Blanco,
    Negro
}
impl ConcesionarioAuto{
    fn new(nombre: String, direccion: String, capacidad_maxima: u8, autos: Vec<Auto>) -> ConcesionarioAuto {
        ConcesionarioAuto{
            nombre,
            direccion,
            capacidad_maxima,
            autos,
        }
    }
    fn agregar_auto(&mut self, auto: Auto) -> bool{
        if(self.autos.len() < self.capacidad_maxima.into()){
            self.autos.push(auto);
            return true;
        }else{
            return false;
        }
    }
    fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        if let Some(pos) = self.autos.iter().position(|a| a == auto) {
            self.autos.remove(pos);
            true
        } else {
            false
        }
    }
    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|&a| a == auto)
    }
}
fn main() {
    // let auto1 = Auto {
    //     marca: "Toyota".to_string(),
    //     modelo: "Corolla".to_string(),
    //     anio: 2020,
    //     precio: 20000.0,
    //     color: Color::Blanco,
    // };

    // let auto2 = Auto {
    //     marca: "Ford".to_string(),
    //     modelo: "Focus".to_string(),
    //     anio: 2019,
    //     precio: 18000.0,
    //     color: Color::Negro,
    // };

    // let mut concesionario = ConcesionarioAuto::new(
    //     "Concesionario Ejemplo".to_string(),
    //     "Calle Principal 123".to_string(),
    //     10,
    //     vec![],
    // );

    // println!("Agregando auto 1: {}", concesionario.agregar_auto(auto1.clone()));
    // println!("Agregando auto 2: {}", concesionario.agregar_auto(auto2.clone()));

    // println!("Buscando auto 1:");
    // if let Some(auto) = concesionario.buscar_auto(&auto1) {
    //     println!("Encontrado: Marca {}, Modelo {}", auto.marca, auto.modelo);
    // } else {
    //     println!("Auto no encontrado");
    // }

    // println!("Eliminando auto 1: {}", concesionario.eliminar_auto(&auto1));
    // println!("Eliminando auto no existente: {}", concesionario.eliminar_auto(&auto1));
}

#[test]
    fn test_agregar_auto_exitoso() {
        let mut concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            10,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        assert!(concesionario.agregar_auto(auto1.clone()));
        assert_eq!(concesionario.autos.len(), 1);
        assert_eq!(concesionario.autos[0], auto1);
    }

    #[test]
    fn test_agregar_auto_falla_por_capacidad() {
        let mut concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            1,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        let auto2 = Auto {
            marca: "Ford".to_string(),
            modelo: "Focus".to_string(),
            anio: 2019,
            precio: 18000.0,
            color: Color::Negro,
        };

        concesionario.agregar_auto(auto1);
        assert!(!concesionario.agregar_auto(auto2));
        assert_eq!(concesionario.autos.len(), 1);
    }

    #[test]
    fn test_eliminar_auto_exitoso() {
        let mut concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            10,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        concesionario.agregar_auto(auto1.clone());
        assert!(concesionario.eliminar_auto(&auto1));
        assert!(concesionario.autos.is_empty());
    }

    #[test]
    fn test_eliminar_auto_falla_no_existente() {
        let mut concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            10,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        assert!(!concesionario.eliminar_auto(&auto1));
    }

    #[test]
    fn test_buscar_auto_exitoso() {
        let mut concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            10,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        concesionario.agregar_auto(auto1.clone());
        let resultado = concesionario.buscar_auto(&auto1);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap(), &auto1);
    }

    #[test]
    fn test_buscar_auto_falla_no_existente() {
        let concesionario = ConcesionarioAuto::new(
            "Concesionario Ejemplo".to_string(),
            "Calle Principal 123".to_string(),
            10,
            vec![],
        );

        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2020,
            precio: 20000.0,
            color: Color::Blanco,
        };

        let resultado = concesionario.buscar_auto(&auto1);
        assert!(resultado.is_none());
    }
    

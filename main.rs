fn main() { 
    let rayon = 5.0; 
  
    let p = perimetre_cercle(rayon); 
    println!("Le perimetre du cercle est egal e : {}", p); 
  
    let s = surface_cercle(rayon); 
    println!("La surface du cercle est egale a : {}", s); 
  
    let s_sph = surface_sphere(rayon); 
    println!("La surface de la sphere est egale a : {}", s_sph); 
  
    let v_sph = volume_sphere(rayon); 
    println!("Le volume de la sphere est egal a : {}", v_sph); 
 }  
  
 fn perimetre_cercle(rayon: f64) -> f64 { 
    assert!(rayon >= 0.0); 
  
    let perimetre = 2.0 * std::f64::consts::PI * rayon; 
    perimetre 
 }  
  
 fn surface_cercle(rayon: f64) -> f64 { 
    assert!(rayon >= 0.0); 
  
    let surface = std::f64::consts::PI * rayon * rayon; 
    surface 
 }  
  
 fn surface_sphere(rayon: f64) -> f64 { 
    assert!(rayon >= 0.0); 
  
    let surface_sph = 4.0 * std::f64::consts::PI * rayon * rayon; 
    surface_sph 
 }  
  
 fn volume_sphere(rayon: f64) -> f64 { 
    assert!(rayon >= 0.0); 
  
    let volume_sph = 4.0 / 3.0 * 
 std::f64::consts::PI *             f64::powf(rayon, 3.0); 
    volume_sph 
 } 

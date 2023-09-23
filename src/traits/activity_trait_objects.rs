use crate::print_title;

trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

/// Box always has a usize type
fn total_cost(material: &Vec<Box<dyn Material>>) -> f64 {
    material.iter().map(|mat| mat.total_cost()).sum()
}

pub fn master(show: bool) {
    if show {
        print_title("Activity Trait Objects");

        let carpet = Box::new(Carpet(50.0));
        let tile = Box::new(Tile(60.0));
        let wood = Box::new(Wood(70.0));

        let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];

        let total = total_cost(&materials);
        println!("total cost: ${}", total);
    }
}


struct Ratio {
    number: u32,
    total: u32
}

impl Ratio {
    pub const DEFAULT: Ratio = Ratio{number:0,total:0};

    pub fn add(&mut self, succes: bool){
        if succes {
            self.number += 1;
        }
        self.total += 1;
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\t/{}", self.number, self.total)
    }
}

struct Statistics {
    object_intersections: Ratio,
    triangle_intersections: Ratio
}

static mut STATISTICS: Statistics = Statistics {
    object_intersections: Ratio::DEFAULT,
    triangle_intersections: Ratio::DEFAULT
};

pub fn object_intersection(succes: bool) {
    unsafe {
        STATISTICS.object_intersections.add(succes);
    }
}

pub fn triangle_intersection(succes: bool) {
    unsafe {
        STATISTICS.triangle_intersections.add(succes);
    }
}


pub fn print_statistics() {
    unsafe {
        println!("\nSTATISTICS:");
        println!("Object Intersections:\t\t{}", STATISTICS.object_intersections);
        println!("\tTriangle Intersections:\t{}", STATISTICS.triangle_intersections);
    }
}

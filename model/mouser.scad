$fs = 0.1;
$fa = 1.0;

base_thickness = 2.0;
peg_height = 5.0;
peg_dx = 19.5;
peg_dy = 26;
inward_angle=10;
inward_gap=50;
support_d = 15;


for (i = [-1, 1]) {
  scale([i, 1, 1])
difference() {
  union() {
    translate([0, -50, 0])
      cube([50, 100, 2.0], center = false);


    translate([0, -10, base_thickness]) {
      for (y = [-peg_dy/2, peg_dy/2]) {
        translate([peg_dx/2, y, 0])
          cylinder(h = peg_height, d1 = 3.0, d2 = 2.0);
      }
    }

    translate([inward_gap/2, 0, 0])
      rotate([0, 0, inward_angle])
        translate([0, 50, 0]) 
          rotate([90, 0, 0]) 
            cylinder(100, d = support_d);
  }
  
  translate([-100, -100, -10])
    cube([200, 200, 10], center = false);  

  translate([-100, -80, -25])
    cube([200, 50, 50], center = false);

  
  for (i = [1]) {
    //translate([0, 0, base_thickness])
      //rotate([90, 0, i*10])

    
    translate([i*(inward_gap/2), 0, 0])
    rotate([0, 0, i*inward_angle])
    translate([support_d/2, -100, -25])
    cube([200, 200, 50], center = false);
    
    
    
    translate([i*(inward_gap/2), 0, 0])
    rotate([0, 0, i*inward_angle])
    translate([-2.5, -100, base_thickness])
    cube([5, 200, 200]);
 
  }
  
      
}
}
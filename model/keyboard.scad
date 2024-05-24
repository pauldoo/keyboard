//cube();

$fs = 0.1;
$fa = 1;

module switch_hole(d) {
    translate([-d/2, -d/2, -1]) {
      cube([d, d, 3]);
    }
}
if (false) {
// test different switch hole sizes.
difference() {
    cube([100, 40, 1.5]);
    translate([25, 20, 0]) {
        // this was the winner
        switch_hole(13.9);
    }
    translate([50, 20, 0]) {
        switch_hole(14);
    }
    translate([75, 20, 0]) {
        // Dot beside the bigger one so I can tell which is which.
        switch_hole(14.1);
        translate([-1.5, 11, -1]) {
            cube([3, 3, 3]);
        }
    }
}
}

if (false) {
// key letter print test
translate([0, -30, 0]) {

    scale([-1, 1, 1]) {
        difference() {
            cube([15, 15, 3]);
            translate([7.5, 7.5, 0]) {
                linear_extrude(height=3, center=true) {
                    text("P", font="Liberation Mono:style=Bold", halign="center", valign="center");
                }
            }
        }
    }
}
}

module chorner(r, d) {
    translate([0,0,-(d+1)/2]) {
        difference() {
            cube([r+1, r+1, d+1]);
            translate([0,0,-0.5])
            cylinder(d+2, r, r);
        }
    }
}

//chorner(1, 3);

module stem() {
    depth = 8; // socket has 4mm depth, so 4mm pokes out.
    width = 5.6; // 5.5 is basically perfect, minute wobble..? // 6; // 5
    cross_t = 1.5; // 1.25; // 1
    cross_d = 4.25; // 4; // 3
    round_r = 1.25; // 1
    
    translate([0, 0, depth/2])
    difference() {
        cube([width, width, depth], center=true);
        cube([cross_d, cross_t, depth + 1], center=true);
        cube([cross_t, cross_d, depth + 1], center=true);
        multmatrix([
            [1, 0, 0, width/2 - round_r],
            [0, 1, 0, width/2 - round_r],
            [0, 0, 1, 0]]) {
            chorner(round_r, depth);
        }
        multmatrix([
            [1, 0, 0, width/2 - round_r],
            [0, -1, 0, -(width/2 - round_r)],
            [0, 0, 1, 0]]) {
            chorner(round_r, depth);
        }

        multmatrix([
            [-1, 0, 0, -(width/2 - round_r)],
            [0, 1, 0, width/2 - round_r],
            [0, 0, 1, 0]]) {
            chorner(round_r, depth);
        }

        multmatrix([
            [-1, 0, 0, -(width/2 - round_r)],
            [0, -1, 0, -(width/2 - round_r)],
            [0, 0, 1, 0]]) {
            chorner(round_r, depth);
        }


    }
}
stem();


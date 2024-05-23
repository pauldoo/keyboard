//cube();



module switch_hole(d) {
    translate([-d/2, -d/2, -1]) {
      cube([d, d, 3]);
    }
}

// test different switch hole sizes.
difference() {
    cube([100, 40, 1.5]);
    translate([25, 20, 0]) {
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

//cube();

$fs = 0.1;
$fa = 1;

// width of a key face
key_face_d = 15;
// thickness of a key face
key_face_t = 2;
// length of key stem
stem_l = 4;

key_stride = 19;
board_t = 5;
border = 2;

module switch_hole() {
    // make a hold in the center
    d = 13.9;
    translate([-d/2, -d/2, -0.5]) {
      cube([d, d, board_t+1]);
    }
    e = (d + key_stride) / 2;
    translate([-e/2, -e/2, 1.5]) {
      cube([e, e, board_t+1]);
    }
}


module key_face(s) {
    translate([0,0,key_face_t/2]) {
        difference() {
            cube([key_face_d, key_face_d, key_face_t], center=true);
            translate([0, 0, key_face_t - 0.5]) {
                linear_extrude(height=key_face_t, center=true) {
                    text(s, font="Liberation Mono:style=Bold", halign="center", valign="center");
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
    depth = stem_l + 1; // socket has 4mm depth
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

module key_cap() {
    stem();
    translate([0, 0, stem_l]) {
        key_face("P");
    }
    for (a=[0, 90, 180, 270]) {
        rotate(a, [0,0,1]) {
            multmatrix([
                [1,0,-1/(stem_l + key_face_t),key_face_d/2],
                [0,1,0,-key_face_d/2],
                [0,0,1,0]]
            ) {
                cube([1, key_face_d, stem_l + key_face_t]);
            }
            translate([key_face_d/2, key_face_d/2, 0]) {
                difference() {
                    cylinder(stem_l + key_face_t, 1, 0);
                    translate([-1, -1, -0.5]) {
                        cube([1, 1, stem_l + key_face_t + 1]);
                    }
                    
                }
            }
        }
    }
}

module left_board() {
    rows = 4;
    cols = 4;
    difference() {
        cube([
            cols * key_stride + 2 * border,
            rows * key_stride + 2 * border,
            board_t
        ]);
    
        for(r = [0:4]) {
            xo = r % 2 == 0 ? 0 : 0.5;
            for(i = [0:10]) {
                translate([
                    border + (i+xo+0.5) * key_stride,
                    border + (r+0.5)*key_stride,
                    0]
                ) {
                    switch_hole();
                }
            }
       }
    }
}

module right_board() {
}

left_board();

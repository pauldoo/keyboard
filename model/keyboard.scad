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
border = 5;
// row height multiplier for fn key row.
fn_key_fudge=1.5;

// tab width
tab_width = 1.5;
// caps lock
cl_width = 1.75;
// shift
shift_width = 1.25;
// ctrl, super, alt
csa_width = 1.25;
// space_width
space_width = 3.0;

keeb_depth = 20;

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

module switch_hole_strip(n) {
    for (i = [0:(n-1)]) {
        translate([i*key_stride, 0, 0]) {
            switch_hole();
        }
    }
}

module left_board() {
    rows = 6;
    cols = 7.25;

    cube([border, (rows + fn_key_fudge - 1) * key_stride + 2 * border, keeb_depth]);
    translate([
        cols * key_stride + 1 * border,0,0])
        cube([border, (rows + fn_key_fudge - 1) * key_stride + 2 * border, keeb_depth]);
    
    difference() {
        cube([
            cols * key_stride + 2 * border,
            (rows + fn_key_fudge - 1) * key_stride + 2 * border,
            board_t
        ]);
    
        // esc + f1-6
        translate([
            border + 0.5*key_stride,
            border + 0.5*fn_key_fudge*key_stride,
            0]) {
            switch_hole_strip(7);
        }
        
        // ` + 1-6
        translate([
            border + 0.5*key_stride,
            border + (fn_key_fudge + 0.5)*key_stride,
            0]) {
            switch_hole_strip(7);
        }

        // tab + qwert row
        translate([
            border,
            border + (fn_key_fudge + 1.5)*key_stride,
            0]) {
            translate([tab_width/2 * key_stride ,0,0])
            switch_hole();
            translate([(tab_width+0.5) * key_stride ,0,0]) 
            switch_hole_strip(5);
        }

        // capslock + asdfg row
        translate([
            border,
            border + (fn_key_fudge + 2.5)*key_stride,
            0]) {
            translate([cl_width/2 * key_stride ,0,0]) switch_hole();
            translate([(cl_width+0.5) * key_stride ,0,0]) switch_hole_strip(5);
        }
        // shift + \zxcvb
        translate([
            border,
            border + (fn_key_fudge + 3.5)*key_stride,
            0]) {
            translate([shift_width/2 * key_stride ,0,0]) switch_hole();
            translate([(shift_width+0.5) * key_stride ,0,0]) switch_hole_strip(6);
        }
        
        // ctrl, super, alt, space
        translate([
            border,
            border + (fn_key_fudge + 4.5)*key_stride,
            0]) {
            translate([csa_width*0.5 * key_stride ,0,0]) switch_hole();
            translate([csa_width*1.5 * key_stride ,0,0]) switch_hole();
            translate([csa_width*2.5 * key_stride ,0,0]) switch_hole();
            translate([(csa_width*3.0 + space_width/2) * key_stride, 0, 0]) switch_hole();
        }

    }
}

module right_board() {
    rows = 6.0;
    cols = 9.5;

    cube([border, (rows + fn_key_fudge - 1) * key_stride + 2 * border, keeb_depth]);
    translate([
        cols * key_stride + 1 * border,0,0])
        cube([border, (rows + fn_key_fudge - 1) * key_stride + 2 * border, keeb_depth]);
    
    difference() {
        cube([
            cols * key_stride + 2 * border,
            (rows + fn_key_fudge - 1) * key_stride + 2 * border,
            board_t
        ]);
        
        // f6-12, vol +/0, del
        translate([
            border + 1.0*key_stride,
            border + 0.5*fn_key_fudge*key_stride,
            0]) {
            switch_hole_strip(9);
        }

        // 7-0, -/+, backspace (2 wide), home
        translate([
            border + 1.0*key_stride,
            border + (fn_key_fudge + 0.5)*key_stride,
            0]) {
            switch_hole_strip(6);
            translate([6.5 * key_stride ,0,0]) switch_hole();
            translate([8.0 * key_stride ,0,0]) switch_hole();
        }
        // yuiop[], enter-gap, end
        translate([
            border + 0.5*key_stride,
            border + (fn_key_fudge + 1.5)*key_stride,
            0]) {
            switch_hole_strip(7);
            translate([8.5 * key_stride ,0,0]) switch_hole();
        }
        
        // enter
        translate([
            border + 7.875 * key_stride,
            border + (fn_key_fudge + 2.0)*key_stride,
            0])
            switch_hole();
        
        
        // hjkl;'#, enter-gap, pgup
        translate([
            border + 0.75*key_stride,
            border + (fn_key_fudge + 2.5)*key_stride,
            0]) {
            switch_hole_strip(7);
            translate([8.25 * key_stride ,0,0]) switch_hole();
        }
        
        // nm,./, shift (1.5), up, pgdn
        translate([
            border + 1.25*key_stride,
            border + (fn_key_fudge + 3.5)*key_stride,
            0]) {
            switch_hole_strip(5);
            translate([(5-1 + 6.75)/2 * key_stride ,0,0]) switch_hole();
            translate([6.75 * key_stride ,0,0]) switch_hole();
            translate([7.75 * key_stride ,0,0]) switch_hole();
        }
        
        // space, alt, super, ctrl, left, down, right, 
        translate([
            border + (cols-6-space_width)*key_stride,
            border + (fn_key_fudge + 4.5)*key_stride,
            0]) {
            translate([(space_width/2) * key_stride,0,0]) switch_hole();
            translate([(space_width+0.5) * key_stride,0,0]) switch_hole_strip(6);
        }
    }
}

left_board();

translate([8*key_stride + border*2, 0, 0])
right_board();

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
right_shift_width = 1.75;

keeb_depth = 20;

font_name="MesloLGL Nerd Font";

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


module key_face(s1, s2=undef, width=1.0) {
    translate([0,0,key_face_t/2]) {
        difference() {            
            cube([key_face_d + (width-1.0)*key_stride, key_face_d, key_face_t], center=true);
            
            po = (s2 != undef) ? -0.125 : 0;
            base_font_size = (s2 != undef) ? 7 : 12;
            
            translate([0, key_face_d*po, key_face_t - 0.5]) {
                linear_extrude(height=key_face_t, center=true) {
                    ts = base_font_size/max(2, len(s1));
                    text(s1, size=ts,
                        font=font_name, 
                        halign="center", valign="center");
                }
            }

            if (s2 != undef) {
                translate([0, key_face_d*0.25, key_face_t - 0.5]) {
                    linear_extrude(height=key_face_t, center=true) {
                        ts = base_font_size/max(2, len(s2));
                        text(s2, size=ts,
                            font=font_name, 
                            halign="center", valign="center");
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

module key_wall(x, y) {
    multmatrix([
        [1,0,-1/(stem_l + key_face_t),(key_face_d + (x-1)*key_stride)/2],
        [0,1,0,-(key_face_d + (y-1)*key_stride)/2],
        [0,0,1,0]]
    ) {
        cube([1, key_face_d + (y-1)*key_stride, stem_l + key_face_t]);
    }
    translate([(key_face_d + (x-1)*key_stride)/2, (key_face_d + (y-1)*key_stride)/2, 0]) {
        difference() {
            cylinder(stem_l + key_face_t, 1, 0);
            translate([-1, -1, -0.5]) {
                cube([1, 1, stem_l + key_face_t + 1]);
            }
            
        }
    }
}

module key_cap(s1, s2, width) {
    stem();
    translate([0, 0, stem_l]) {
        key_face(s1, s2, width);
    }
    for (a=[0, 180]) {
        rotate(a, [0,0,1]) {
            key_wall(width, 1.0);
        }
    }
    for (a=[90, 270]) {
        rotate(a, [0,0,1]) {
            key_wall(1.0, width);
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
        
        // nm,./, shift (1.75), up, pgdn
        translate([
            border + 1.25*key_stride,
            border + (fn_key_fudge + 3.5)*key_stride,
            0]) {
            switch_hole_strip(5);
            // todo - need a variable for right-shift width
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

module key_row(labels, idx) {
    if (idx < len(labels)) {
        width = len(labels[idx]) < 3 ? 1.0 : labels[idx][2];
        translate([key_stride * width / 2, 0, 0])
        key_cap(labels[idx][0], labels[idx][1], width);
        
        translate([key_stride * width, 0, 0])
        key_row(labels, idx+1);
    }
}

module key_rows(labels_list) {
    for (j = [0 : len(labels_list)-1]) {
        labels = labels_list[j];
        translate([0, -key_stride *j, 0])
        key_row(labels, 0);
    }
}

module all_keys() {
    key_rows([
        [
            ["ESC"],
            ["F1", "!"],
            ["F2", "\""],
            ["F3", "£"],
            ["F4", "$"],
            ["F5", "%"],
            ["F6", "^"],
            ["F7", "&"],
            ["F8", "*"],
            ["F9", "("],
            ["F10", ")"],
            ["F11", "_"],
            ["F12", "+"],
            ["\U00f027"],
            ["\U00f028"],
            ["del"]
        ],    
        [
            ["`", "¬"],
            ["1", "!"],
            ["2", "\""],
            ["3", "£"],
            ["4", "$"],
            ["5", "%"],
            ["6", "^"],
            ["7", "&"],
            ["8", "*"],
            ["9", "("],
            ["0", ")"],
            ["-", "_"],
            ["=", "+"],
            ["\U0f0b5c", undef, 2.0],
            ["home"]
        ], [
            ["\U0f0312", undef, tab_width],
            ["Q"],
            ["W"],
            ["E"],
            ["R"],
            ["T"],
            ["Y"],
            ["U"],
            ["I"],
            ["O"],
            ["P"], 
            ["[", "}"], 
            ["]", "}"],
            ["end"]
        ], [
            ["\U0f0632", undef, cl_width],
            ["A"],
            ["S"],
            ["D"],
            ["F"],
            ["G"],
            ["H"],
            ["J"],
            ["K"],
            ["L"],
            [";", ":"], 
            ["'", "@"], 
            ["#", "~"],
            ["pgup"]
        ], [
            ["\U0f0636", undef, shift_width],
            ["\\", "|"],
            ["Z"],
            ["X"],
            ["C"],
            ["V"],
            ["B"],
            ["N"],
            ["M"],
            [",", "<"],
            [".", ">"], 
            ["/", "?"],
            ["\U0f0636", undef, right_shift_width],
            ["\U00f062", undef],
            ["pgdn"]
        ], [
            ["CTRL", undef, csa_width],
            ["SUP", undef, csa_width],
            ["ALT", undef, csa_width],
            ["", undef, space_width],
            ["", undef, space_width],
            ["ALT", undef],
            ["SUP", undef],
            ["CTRL", undef],
            ["\U00f060", undef],
            ["\U00f063", undef],
            ["\U00f061", undef]
        ]
    ]);
}

left_board();

translate([8*key_stride + border*2, 0, 0]) right_board();

translate([0, -2 * key_stride, 0]) all_keys();

k = 6.0;
//k = k + 1;
echo(k);


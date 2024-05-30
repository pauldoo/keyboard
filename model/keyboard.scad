//cube();


$fs = 0.1;
$fa = 5;

// width of a key face
key_face_d = 15;
// thickness of a key face
key_face_t = 2;
// length of key stem
stem_l = 4;

key_stride = 19;
board_t = 5;
border = 5;
// row/col multiplier for fn key row and magic keys.
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

enter_top_width = 1.5;
enter_bottom_width = 1.25;
magic_key_width = 1.5; // caps aren't this big, only the space on the board.

keeb_depth = 20;

font_name="MesloLGL Nerd Font:style=Bold";

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


module key_face(s1, s2=undef, width=1.0, bump=false) {
    translate([0,0,key_face_t/2]) {
        if (bump) {
            translate([0, -0.375*key_face_d, key_face_t/2])
            cube([key_face_d/3, 0.5, 1.0], center=true);
        }
        difference() {            
            cube([key_face_d + (width-1.0)*key_stride, key_face_d, key_face_t], center=true);
            
            po = (s2 != undef) ? -0.125 : 0;
            base_font_size = (s2 != undef) ? 7 : 12;
            if (s1 != undef) {
                translate([0, key_face_d*po, key_face_t - 0.5]) {
                    linear_extrude(height=key_face_t, center=true) {
                        ts = base_font_size/max(2, len(s1));
                        text(s1, size=ts,
                            font=font_name, 
                            halign="center", valign="center");
                    }
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

module key_cap(s1, s2, width, bump) {
    stem();
    translate([0, 0, stem_l]) {
        key_face(s1, s2, width, bump);
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

module enter_key() {
    // Bodge...
    // Simply take the union of a bunch of keys, then fixup space for the stem.

    difference() {
        union() {
            translate([-(enter_top_width-enter_bottom_width)*key_stride/2, key_stride/2, 0])
            key_cap("\U0f0311", width=enter_top_width);
         
            translate([0, -key_stride/2, 0])   
            key_cap(undef, width=enter_bottom_width);

            key_cap(undef, width=enter_bottom_width);
        }
        //sphere(r=10);
        cutout_x = key_face_d + (enter_bottom_width-1) * key_stride - 2;
        cutout_y = key_face_d + key_stride - 2;
        translate([
            -cutout_x / 2, 
            -cutout_y / 2, 
            0])
        cube([
            cutout_x, 
            cutout_y,
            stem_l]);
    }
    
    stem();
 }

module switch_holes(widths, ltr, idx=undef) {
    if (idx == undef) {
        if (ltr) {
            switch_holes(widths, ltr, idx=0);
        } else {
            switch_holes(widths, ltr, idx=len(widths)-1);
        }
    } else {
        if (ltr && idx < len(widths)) {
            w = widths[idx][0] == undef ? widths[idx] : widths[idx][0];
            
            if (widths[idx][0] == undef) {
                translate([w * 0.5 * key_stride, 0, 0])
                switch_hole();
            }
            translate([w * 1.0 * key_stride, 0, 0])
            switch_holes(widths, ltr, idx+1);
        }
        if (!ltr && idx >= 0) {
            w = widths[idx][0] == undef ? widths[idx] : widths[idx][0];
            
            if (widths[idx][0] == undef) {
                translate([-w * 0.5 * key_stride, 0, 0])
                switch_hole();
            }
            translate([-w * 1.0 * key_stride, 0, 0])
            switch_holes(widths, ltr, idx-1);
        }
    }
}

module left_board() {
    rows = 6;
    cols = 7.25 + 0.5 + magic_key_width/2;

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
    
        translate([border + (-magic_key_width/2 + 0.5) * key_stride, 0, 0]) {
            // esc + f1-6
            translate([0, border + 0.5*fn_key_fudge*key_stride,0])
            switch_holes([[magic_key_width], 1, 1, 1, 1, 1, 1, 1], true);

            // ` + 1-6
            translate([0, border + (fn_key_fudge + 0.5)*key_stride,0])
            switch_holes([magic_key_width, 1, 1, 1, 1, 1, 1, 1], true);

            // tab + qwert row
            translate([0, border + (fn_key_fudge + 1.5)*key_stride,0])
            switch_holes([magic_key_width, tab_width, 1, 1, 1, 1, 1], true);

            // capslock + asdfg row
            translate([0, border + (fn_key_fudge + 2.5)*key_stride,0])
            switch_holes([magic_key_width, cl_width, 1, 1, 1, 1, 1], true);

            // shift + \zxcvb
            translate([0, border + (fn_key_fudge + 3.5)*key_stride,0])
            switch_holes([magic_key_width, shift_width, 1, 1, 1, 1, 1, 1], true);

            // ctrl, super, alt, space
            translate([0, border + (fn_key_fudge + 4.5)*key_stride,0])
            switch_holes([magic_key_width, csa_width, csa_width, csa_width, space_width], true);
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

        translate([border + cols * key_stride, 0, 0]) {

            // f6-12, vol +/0, del
            translate([0, border + 0.5*fn_key_fudge*key_stride, 0])
            switch_holes([1, 1, 1, 1, 1, 1, 1, 1, 1], false);

            // 7-0, -/+, backspace (2 wide), home
            translate([0, border + (fn_key_fudge + 0.5)*key_stride, 0])
            switch_holes([1, 1, 1, 1, 1, 1, 2, 1], false);

            // yuiop[], enter-gap, end
            translate([0, border + (fn_key_fudge + 1.5)*key_stride, 0])
            switch_holes([1, 1, 1, 1, 1, 1, 1, [enter_top_width], 1], false);

            // enter
            translate([
                -(1+enter_bottom_width/2)*key_stride,
                border + (fn_key_fudge + 2.0)*key_stride,
                0])
                switch_hole();

            // hjkl;'#, enter-gap, pgup
            translate([0, border + (fn_key_fudge + 2.5)*key_stride, 0])
            switch_holes([1, 1, 1, 1, 1, 1, 1, [enter_bottom_width], 1], false);

            // nm,./, shift (1.75), up, pgdn
            translate([0, border + (fn_key_fudge + 3.5)*key_stride, 0])
            switch_holes([1,1,1,1,1, right_shift_width, 1, 1], false);

            // space, alt, super, ctrl, left, down, right, 
            translate([0, border + (fn_key_fudge + 4.5)*key_stride, 0])
            switch_holes([space_width, 1, 1, 1, 1, 1, 1], false);
        }
    }
}

module key_rows(labels, idx=0, cumulative_width=0, max_width=200) {
    if (idx < len(labels)) {
        width = len(labels[idx]) < 3 ? 1.0 : labels[idx][2];
        bump = len(labels[idx]) < 4 ? false : labels[idx][3];

        cw = cumulative_width + key_stride*width;
        if (cw >= max_width) {
            translate([0, -key_stride, 0])
            key_rows(labels, idx, 0, max_width);
        } else {

            translate([cumulative_width + key_stride * width / 2, 0, 0])
            key_cap(labels[idx][0], labels[idx][1], width, bump);
            
            key_rows(labels, idx+1, cw, max_width);
        }
    }
}

module all_keys() {
    key_rows([
            ["ESC"],
            ["\U0f12ab"],
            ["\U0f12ac"],
            ["\U0f12ad"],
            ["\U0f12ae"],
            ["\U0f12af"],
            ["\U0f12b0"],
            ["\U0f12b1"],
            ["\U0f12b2"],
            ["\U0f12b3"],
            ["\U0f12b4"],
            ["\U0f12b5"],
            ["\U0f12b6"],
            ["\U00f027"], // voldown
            ["\U00f028"], // volup
            ["Del"],

            ["\U0f0bed"],
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
            ["Home"],

            ["\U0f0bf0"],
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
            ["End"],

            ["\U0f0bf3"],
            ["\U0f0632", undef, cl_width],
            ["A"],
            ["S"],
            ["D"],
            ["F", undef, 1.0, true],
            ["G"],
            ["H"],
            ["J", undef, 1.0, true],
            ["K"],
            ["L"],
            [";", ":"], 
            ["'", "@"], 
            ["#", "~"],
            ["PgUp"],

            ["\U0f0bf6"],
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
            ["\U00eaa1", undef], // up
            ["PgDn"],

            ["\U0f0bf9"],
            ["Ctrl", undef, csa_width],
            ["\U00e712", undef, csa_width], // super
            ["Alt", undef, csa_width],
            ["", undef, space_width],
            ["", undef, space_width],
            ["Alt", undef],
            ["\U00e712", undef],
            ["Ctrl", undef],
            ["\U00ea9b", undef], // left
            ["\U00ea9a", undef], // down
            ["\U00ea9c", undef] // right
    ]);
}

left_board();

translate([9*key_stride + border*2, 0, 0]) right_board();

translate([0, -2 * key_stride, 0]) all_keys();

translate([-1*key_stride, 0, 0]) enter_key();


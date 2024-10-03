#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct GlobalConst {
    uint a;
    uvec3 b;
    int c;
};
struct AlignedWrapper {
    int value;
};
struct Baz {
    mat3x2 m;
};
struct MatCx2InArray {
    mat4x2 am[2];
};

float read_from_private(inout float foo_2) {
    float _e1 = foo_2;
    return _e1;
}

float test_arr_as_arg(float a[5][10]) {
    return a[4][9];
}

void assign_through_ptr_fn(inout uint p) {
    p = 42u;
    return;
}

void assign_array_through_ptr_fn(inout vec4 foo_3[2]) {
    foo_3 = vec4[2](vec4(1.0), vec4(2.0));
    return;
}

int array_by_value(int a_1[5], int i) {
    return a_1[i];
}

vec4 matrix_by_value(mat4x4 m, int i_1) {
    return m[i_1];
}

void main() {
    uint val = 33u;
    vec4 arr[2] = vec4[2](vec4(6.0), vec4(7.0));
    assign_through_ptr_fn(val);
    assign_array_through_ptr_fn(arr);
    return;
}


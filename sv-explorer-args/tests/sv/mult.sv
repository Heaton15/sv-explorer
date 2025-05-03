module mult (  /*AUTOARG*/
    // Outputs
    z,
    // Inputs
    a,
    b
);
  input [3:0] a, b;
  output [7:0] z;

  assign z = a * b;
endmodule

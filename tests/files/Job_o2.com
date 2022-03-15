%chk=/scratch/scr/ybyygu//o2.chk
%nproc=4
%mem=2GB
#p 6-311+g(d,p) geom=connectivity b3lyp iop(5/33=1) nosymm extraoverlay

8/7=1,10=4/1;
9/16=-3/6;
6//8;

Title Card Required

0    3
O                 -3.33902648   -1.01195558    0.00000000
O                 -4.50062648   -1.01195558    0.00000000
 
1 2 2.0
2

100
205
402
 
 

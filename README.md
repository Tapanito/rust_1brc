# PC Specs

The main spec relevant for the single threaded method is the SSD performance.

```
 fio --name TEST --eta-newline=5s --filename=fio-tempfile.dat --rw=read --size=500m --io_size=10g --blocksize=1024k --ioengine=libaio --fsync=10000 --iodepth=32 --direct=1 --numjobs=1 --runtime=60 --group_reporting
TEST: (g=0): rw=read, bs=(R) 1024KiB-1024KiB, (W) 1024KiB-1024KiB, (T) 1024KiB-1024KiB, ioengine=libaio, iodepth=32
fio-3.28
Starting 1 process
TEST: Laying out IO file (1 file / 500MiB)
Jobs: 1 (f=1): [R(1)][100.0%][r=2489MiB/s][r=2489 IOPS][eta 00m:00s]
TEST: (groupid=0, jobs=1): err= 0: pid=53600: Sun Apr 14 21:29:32 2024
  read: IOPS=2487, BW=2488MiB/s (2609MB/s)(10.0GiB/4116msec)
    slat (usec): min=12, max=677, avg=30.57, stdev=21.86
    clat (usec): min=477, max=24797, avg=12770.79, stdev=2572.45
     lat (usec): min=492, max=24816, avg=12801.43, stdev=2572.43
    clat percentiles (usec):
     |  1.00th=[ 2507],  5.00th=[ 9110], 10.00th=[11600], 20.00th=[12387],
     | 30.00th=[12780], 40.00th=[12780], 50.00th=[12911], 60.00th=[13042],
     | 70.00th=[13042], 80.00th=[13304], 90.00th=[13566], 95.00th=[16319],
     | 99.00th=[22414], 99.50th=[23200], 99.90th=[24249], 99.95th=[24249],
     | 99.99th=[24511]
   bw (  MiB/s): min= 2466, max= 2522, per=100.00%, avg=2488.50, stdev=18.10, samples=8
   iops        : min= 2466, max= 2522, avg=2488.50, stdev=18.10, samples=8
  lat (usec)   : 500=0.11%, 750=0.09%, 1000=0.20%
  lat (msec)   : 2=0.39%, 4=1.15%, 10=4.17%, 20=91.70%, 50=2.20%
  cpu          : usr=0.83%, sys=8.34%, ctx=10162, majf=0, minf=8205
  IO depths    : 1=0.2%, 2=0.4%, 4=0.8%, 8=1.6%, 16=3.3%, 32=93.6%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=99.8%, 8=0.0%, 16=0.0%, 32=0.2%, 64=0.0%, >=64=0.0%
     issued rwts: total=10240,0,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=32

Run status group 0 (all jobs):
   READ: bw=2488MiB/s (2609MB/s), 2488MiB/s-2488MiB/s (2609MB/s-2609MB/s), io=10.0GiB (10.7GB), run=4116-4116msec

Disk stats (read/write):
  nvme0n1: ios=19723/4, merge=0/9, ticks=242842/7, in_queue=242855, util=97.61%

```

# Results

Single Thread performance: 
- Average duration: 374128ms (or 374.128seconds)

[x] Change chunk types from HashMap to Vec
[x] Draw pixels to a texture before drawing to the rendertarget to minimize drawcalls
   -- Currently 1 texture per chunk is drawn. This should be good enough
  that means a maximum of ~8 - 12 drawcalls per frame vs thousands of calls ( 1 per pixel)
[x] Update texture only if the chunk has changed
[x] Change chunk value from enum PixelType to struct Pixel
  -- Required for more complex behaviour like heat
[] Add way to skip pixels that do not require updating
[] Add world gen
[] Implement parallelization

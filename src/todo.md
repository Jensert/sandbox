[x] Change chunk types from HashMap to Vec
[x] Draw pixels to a texture before drawing to the rendertarget to minimize drawcalls
   -- Currently 1 texture per chunk is drawn. This should be good enough
  that means a maximum of ~8 - 12 drawcalls per frame vs thousands of calls ( 1 per pixel)
[x] Update texture only if the chunk has changed
[x] Change chunk value from enum PixelType to struct Pixel
  -- Required for more complex behaviour like heat
[x] Add world gen
[x] add generate_world() function to MapGenerator
    Loop over the layer_rules, and call generate() function on all of them
[x] finish push_layer() function in MapGenerator struct
[x] finish all default layer implementations in MapLayerRule.from_default_layer() function
[x] Finish shader texture implementation
[x] Zoom
[] Add decay mechanic to avoid always falling and chaotic worlds
[] Add gameplay
[x] Move UI logic and drawing to a seperate struct. This struct should be owned by App.
[] Add way to skip pixels that do not require updating
[] Implement parallelization

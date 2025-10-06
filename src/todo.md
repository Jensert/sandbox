# Moving pixels from chunk to chunk
Several options:
- GridMovement struct to support chunk keys in addition to chunk coordinates
  Chunks then update the same way as currently, however in addition they also check if the new position is out of bounds
  If it is, then move the pixel to the neighbouring chunk
  This is probably best

  Currently getting all cross-chunk movements is ready
  Just have to apply the cross chunk movements from the ChunkGrid structs update function
  Need some helper functions

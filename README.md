# space_alloc

General purpose space allocators.
These don't actually allocate any memory, they only provide bookkeeping. Allocation methods will return [Allocation]s with an offset and size that the user can interpret in whichever way they need, for instance, GPU space division.

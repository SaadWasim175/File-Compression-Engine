# File-Compression-Engine
File compression engine made with algorithms like Huffman, RLE and LZW. 
The compression engine works on bytes of any file, allowing it to work on any file, although compression does tend to vary on the file's data itself. For example, more complex images usually don't compress that well, while images with more repetitive data (like an image where the same color occurs multiple times) will compress quite well.

## Compressed file
It returns a compressed file with the extension ".compr", metadata is stored in the starting few bytes of the file. File info stored includes: 

- file format of uncompressed file
- (if image type) width and height of original image

Specific file format metadata for Huffman compression includes:

- identifier (5 bytes)
- original size (8 bytes)
- file format length (1 byte)
- file format itself (10 bytes)
In total, 24 bytes.

## Compression
Currently, for smaller files it does straightforward, simple compression, while for files that would be too large to load in RAM, it compresses them using buffered compression.


## To-Do tasks
Still need to implement a pipeline so that multiple compression algorithms can work in a downstream way.

Need to implement buffered Huffman compression. (Done ✅)

Need to implement buffered Huffman decompression.

Need to build a GUI or a more presentable TUI (terminal user interface).

Need to make different profiles for different kinds of files for better optimized compression (for example, adding lossy compression for image and video types for much better compression of those files).

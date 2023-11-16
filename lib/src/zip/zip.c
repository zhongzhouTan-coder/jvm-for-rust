#include <zlib.h>
#include <string.h>

int zip_inflate(void *outBuf, long long outlen, void *inBuf, long long inlen, char **pmsg)
{
	z_stream strm;
	memset(&strm, 0, sizeof(z_stream));

	*pmsg = 0;

	if (inflateInit2(&strm, MAX_WBITS) != Z_OK)
	{
		*pmsg = strm.msg;
		return 0;
	}

	strm.next_out = (Bytef *)outBuf;
	strm.avail_out = (uInt)outlen;
	strm.next_in = (Bytef *)inBuf;
	strm.avail_in = (uInt)inlen;

	do
	{
		switch (inflate(&strm, Z_PARTIAL_FLUSH))
		{
		case Z_OK:
			break;
		case Z_STREAM_END:
			if (strm.total_out != (uInt)outlen)
			{
				*pmsg = "INFLATER_inflateFully: Unexpected end of stream";
				inflateEnd(&strm);
				return 0;
			}
			break;
		case Z_DATA_ERROR:
			*pmsg = "INFLATER_inflateFully: Compressed data corrupted";
			inflateEnd(&strm);
			return 0;
		case Z_MEM_ERROR:
			*pmsg = "INFLATER_inflateFully: out of memory";
			inflateEnd(&strm);
			return 0;
		default:
			*pmsg = "INFLATER_inflateFully: internal error";
			inflateEnd(&strm);
			return 0;
		}

	} while (strm.avail_in > 0);

	inflateEnd(&strm);
	return 1;
}

size_t zip_deflate(void *outBuf, long long outLen, void *inBuf, long long inLen, char **pmsg)
{
	z_stream strm;
	memset(&strm, 0, sizeof(strm));
	*pmsg = 0;
	size_t result = 0;
	int err_def = deflateInit2(&strm, Z_DEFAULT_COMPRESSION, Z_DEFLATED, MAX_WBITS, 8, Z_DEFAULT_STRATEGY);

	if (err_def == Z_MEM_ERROR)
	{
		*pmsg = "Out of memory in deflateInit2";
		return 0;
	}

	if (err_def != Z_OK)
	{
		*pmsg = "Internal error in deflateInit2";
		return 0;
	}

	strm.next_out = (Bytef *)outBuf;
	strm.avail_out = (uInt)outLen;
	strm.next_in = (Bytef *)inBuf;
	strm.avail_in = (uInt)inLen;

	int err = deflate(&strm, Z_FINISH);

	if (err == Z_OK || err == Z_BUF_ERROR)
	{
		*pmsg = "Buffer too small";
	}
	else if (err != Z_STREAM_END)
	{
		*pmsg = "Intern deflate error";
	}
	else
	{
		result = (size_t)strm.total_out;
	}
	deflateEnd(&strm);
	return result;
}
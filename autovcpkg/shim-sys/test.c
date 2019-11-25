#include <crc32c/crc32c.h>

int crc_it()
{
  return crc32c_value("autovcpkg", strlen("autovcpkg"));
}

require 'mkmf'

LIB_DIRS = [
  RbConfig::CONFIG['libdir'],
  File.expand_path('../../../../rust/target/release', __FILE__)
]

dir_config('faster_blank', nil, LIB_DIRS)

$LOCAL_LIBS << '-lfaster_blank'

create_makefile 'faster_blank'

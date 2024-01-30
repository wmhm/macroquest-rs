#include "macroquest-sys/include/eqlib.h"
#include "macroquest-sys/src/lib.rs.h"

namespace mqrust
{
    namespace eqlib
    {
        rust::Str PlayerClient::name() const { return this->Name; }

        rust::Str EQGroundItem::name() const { return this->Name; }
    }

}

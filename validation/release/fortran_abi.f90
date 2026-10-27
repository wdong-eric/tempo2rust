! Measure the compiler's real default integer and C interoperability.
subroutine al_fortran_abi(native_bits, c_bits, value) bind(C)
  use iso_c_binding
  implicit none
  integer(c_int), intent(out) :: native_bits, c_bits
  integer(c_int), intent(inout) :: value
  native_bits = storage_size(1)
  c_bits = storage_size(1_c_int)
  value = value + 1
end subroutine

# frozen_string_literal: true

require "test_helper"

class FoxcallTest < Test::Unit::TestCase
  test "VERSION" do
    assert do
      ::Foxcall.const_defined?(:VERSION)
    end
  end

  test "something useful" do
    assert_equal("expected", "actual")
  end
end

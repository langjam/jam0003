{
	inputs = {
		nixCargoIntegration.url = "github:yusdacra/nix-cargo-integration";
	};
	outputs = inputs: inputs.nixCargoIntegration.lib.makeOutputs {
		root = ./.;
		overrides = {
			shell = common: prev: with common.pkgs; {
				env = prev.env ++ [
					# Use Mold Linker (its faster)
					{
						name = "RUSTFLAGS";
						value =
							if common.pkgs.stdenv.isLinux
							then "-C link-arg=-fuse-ld=mold -C target-cpu=native -Clink-arg=-Wl,--no-rosegment"
							else "";
					}
				];
			};
		};
	};
}
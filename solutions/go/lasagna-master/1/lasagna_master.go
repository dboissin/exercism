package lasagna

func PreparationTime(layers []string, avgLayerTime int) int {
	if avgLayerTime == 0 {
		avgLayerTime = 2
	}
	return len(layers) * avgLayerTime
}

func Quantities(layers []string) (int, float64) {
	noodles := 0
	sauce := 0.0
	for i := 0; i < len(layers); i++ {
		switch layers[i] {
		case "noodles":
			noodles += 50
		case "sauce":
			sauce += 0.2
		}
	}
	return noodles, sauce
}

func AddSecretIngredient(friendList, ownList []string) {
	ownList[len(ownList)-1] = friendList[len(friendList)-1]
}

func ScaleRecipe(quantities []float64, portions int) []float64 {
	res := make([]float64, len(quantities))
	for i := 0; i < len(quantities); i++ {
		res[i] = quantities[i] / 2 * float64(portions)
	}
	return res
}

object GradleConfigs {
    const val compileSdk = 34
    const val minSdk = 26
    const val ndkVersion = "26.1.10909125"
    const val baseNamespace = "org.ibadalrahman"
    const val packageVersion = "0.1.0"

    fun subNamespace(sub: String) = "$baseNamespace.$sub"
}

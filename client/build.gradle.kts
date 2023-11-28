plugins {
    id("java")
    application
}

application {
    mainClass.set("com.SaHHiiLL.github.Main")
}

group = "com.SaHHiiLL.github"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.9.1"))
    testImplementation("org.junit.jupiter:junit-jupiter")
//    implementation('com.formdev:flatlaf:3.1')
    implementation("com.formdev:flatlaf:3.1")
    // add gson
    implementation("com.google.code.gson:gson:2.8.9")
}

tasks.test {
    useJUnitPlatform()
}
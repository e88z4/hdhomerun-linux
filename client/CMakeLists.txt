cmake_minimum_required(VERSION 3.21)

project(hdhomerun-linux-player-client VERSION 0.1.0 LANGUAGES CXX)

include(CTest)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

find_package(Qt6 6.4 REQUIRED COMPONENTS DBus Multimedia Network Quick QuickControls2 Test)

qt_standard_project_setup(REQUIRES 6.4)
if(COMMAND qt_policy)
    qt_policy(SET QTP0004 NEW)
endif()

qt_add_executable(hdhomerun-linux-player
    src/main.cpp
    src/appcontroller.cpp
    src/appcontroller.h
    src/backendlaunchconfig.cpp
    src/backendlaunchconfig.h
    src/channelnavigation.cpp
    src/channelnavigation.h
    src/dvrworkspacehelpers.cpp
    src/dvrworkspacehelpers.h
    src/idleinhibitor.cpp
    src/idleinhibitor.h
)

qt_add_qml_module(hdhomerun-linux-player
    URI HDHomeRun.Client
    VERSION 1.0
    NO_IMPORT_SCAN
    QML_FILES
        qml/Main.qml
        qml/components/ChannelRail.qml
        qml/components/DvrDeleteDialog.qml
        qml/components/DvrDetailsPanel.qml
        qml/components/DvrRecordingsPanel.qml
        qml/components/DvrRuleEditorDialog.qml
        qml/components/DvrStatusBanner.qml
        qml/components/DvrUpcomingPanel.qml
        qml/components/GuideGrid.qml
        qml/components/IconButton.qml
        qml/components/PlaybackStage.qml
        qml/components/DiagnosticsDrawer.qml
        qml/components/ThemeButton.qml
)

target_link_libraries(hdhomerun-linux-player
    PRIVATE
    Qt6::DBus
    Qt6::Network
    Qt6::Multimedia
    Qt6::Quick
    Qt6::QuickControls2
)

install(TARGETS hdhomerun-linux-player
    BUNDLE DESTINATION .
    RUNTIME DESTINATION bin
)

if(BUILD_TESTING)
    qt_add_executable(backendlaunchconfig-tests
        src/backendlaunchconfig.cpp
        src/backendlaunchconfig.h
        tests/backendlaunchconfig_tests.cpp
    )
    target_include_directories(backendlaunchconfig-tests PRIVATE src)
    target_link_libraries(backendlaunchconfig-tests
        PRIVATE
        Qt6::Network
        Qt6::Test
    )

    qt_add_executable(channelnavigation-tests
        src/channelnavigation.cpp
        src/channelnavigation.h
        tests/channelnavigation_tests.cpp
    )
    target_include_directories(channelnavigation-tests PRIVATE src)
    target_link_libraries(channelnavigation-tests
        PRIVATE
        Qt6::Core
        Qt6::Test
    )

    qt_add_executable(dvrworkspacehelpers-tests
        src/dvrworkspacehelpers.cpp
        src/dvrworkspacehelpers.h
        tests/dvrworkspacehelpers_tests.cpp
    )
    target_include_directories(dvrworkspacehelpers-tests PRIVATE src)
    target_link_libraries(dvrworkspacehelpers-tests
        PRIVATE
        Qt6::Core
        Qt6::Test
    )

    add_test(
        NAME client-offscreen-smoke
        COMMAND ${CMAKE_COMMAND} -E env
            QT_QPA_PLATFORM=offscreen
            HDHR_BACKEND_URL=http://127.0.0.1:9
            HDHR_BACKEND_CMD=/nonexistent
            HDHR_CLIENT_EXIT_AFTER_MS=750
            $<TARGET_FILE:hdhomerun-linux-player>
    )
    add_test(NAME backendlaunchconfig-tests COMMAND backendlaunchconfig-tests)
    add_test(NAME channelnavigation-tests COMMAND channelnavigation-tests)
    add_test(NAME dvrworkspacehelpers-tests COMMAND dvrworkspacehelpers-tests)
    set_tests_properties(client-offscreen-smoke PROPERTIES TIMEOUT 10)
    set_tests_properties(backendlaunchconfig-tests PROPERTIES TIMEOUT 10)
    set_tests_properties(channelnavigation-tests PROPERTIES TIMEOUT 10)
    set_tests_properties(dvrworkspacehelpers-tests PROPERTIES TIMEOUT 10)
endif()
